use axum::Router;
use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::{protocol::cdp::Page, Browser, LaunchOptionsBuilder};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::{env, fs, thread};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define links similar to JavaScript version
    //
    let local_port = 3389;
    let mut links = HashMap::new();
    links.insert("prod".to_string(), "https://cv.hmziq.rs".to_string());
    links.insert(
        "local".to_string(),
        format!("http://127.0.0.1:{}", local_port).to_string(),
    );
    links.insert("dev".to_string(), "http://127.0.0.1:1000".to_string());

    // Parse command line arguments
    let env_arg = env::args().nth(1).unwrap_or_else(|| "local".to_string());
    println!("ENV: {}", env_arg);

    // Start local server if needed
    let server_shutdown_tx = if env_arg == "local" {
        // Define the path to the static files directory - assuming "out" as in the JS version
        let static_dir = "./target/dx/cv/release/web/public";

        // Check if the directory exists
        if !Path::new(static_dir).exists() {
            return Err(format!("Directory '{}' does not exist", static_dir).into());
        }

        // Start the server
        let (tx, server_handle) = start_server(static_dir, local_port).await?;
        println!("Local server started on port {}", local_port);

        // Spawn the server task
        tokio::spawn(server_handle);
        Some(tx)
    } else {
        None
    };

    println!("{} {}", env_arg, links.get(&env_arg).unwrap());

    // Create directories if they don't exist
    let files_dir = Path::new("public/files");
    if !files_dir.exists() {
        fs::create_dir_all(files_dir)?;
    }

    // Launch browser
    let options = LaunchOptionsBuilder::default()
        .headless(true)
        .port(Some(3000))
        .sandbox(false)
        .window_size(Some((1280, 1080)))
        .build()?;

    let browser = Browser::new(options)?;
    println!("Browser launched");

    // Create tab and navigate to URL
    let tab = browser.new_tab()?;

    let url = links.get(&env_arg).unwrap();
    tab.navigate_to(url)?.wait_until_navigated()?;
    println!("Page loaded");

    // Capture dark theme
    capture_pdf(&tab, true)?;
    println!("Captured dark theme");

    // Switch to light theme
    tab.evaluate("toggleTheme();", false)?;

    // Wait for theme to apply
    sleep(4000);

    // Capture light theme
    capture_pdf(&tab, false)?;
    println!("Captured light theme");

    let jpeg_height = get_content_height(&tab)?;

    // true works in github actions
    tab.close(true)?;

    let jpeg_options = LaunchOptionsBuilder::default()
        .headless(true)
        .sandbox(false)
        .window_size(Some((1280, jpeg_height as u32)))
        .build()?;

    let jpeg_browser = Browser::new(jpeg_options)?;

    let jpeg_tab = jpeg_browser.new_tab()?;

    jpeg_tab.navigate_to(url)?.wait_until_navigated()?;
    println!("Page loaded");

    jpeg_tab.evaluate("toggleJpegs();", false)?;
    sleep(1000);

    capture_jpeg(&jpeg_tab, true)?;

    jpeg_tab.evaluate("toggleTheme();", false)?;

    capture_jpeg(&jpeg_tab, false)?;

    // Shutdown the server if it was started
    if let Some(tx) = server_shutdown_tx {
        let _ = tx.send(());
    }

    println!("Browser closed");
    Ok(())
}

// Function to start the local server
async fn start_server(
    static_dir: &str,
    port: u16,
) -> Result<
    (
        tokio::sync::oneshot::Sender<()>, // Channel sender to signal shutdown
        impl std::future::Future<Output = Result<(), std::io::Error>> + Send + 'static, // Server future
    ),
    std::io::Error, // Error type for bind errors etc.
> {
    // Check if the directory exists
    let static_dir_path = Path::new(static_dir);
    if !static_dir_path.exists() {
        eprintln!(
            "Error: Static file directory '{}' does not exist.",
            static_dir
        );
        eprintln!("Please create it and add some files (e.g., index.html).");
        // Return an io::Error that fits the Result signature
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Static file directory '{}' does not exist", static_dir),
        ));
    }
    // Clone the path for use in the async block/future
    let static_dir_owned = static_dir_path.to_path_buf();

    // Define the Axum application router.
    // Serves files from the specified directory.
    let app = Router::new().fallback_service(ServeDir::new(static_dir_owned));

    // Define the address and port the server will listen on.
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // Bind the TCP listener to the address
    let listener = TcpListener::bind(addr).await?; // Use `?` for io::Error propagation
    let actual_addr = listener.local_addr()?; // Get the actual address bound (e.g., if port was 0)
    println!("Static server listening on http://{}", actual_addr);

    // Create a channel for graceful shutdown
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    // Define the server future with graceful shutdown capability
    let server_future = async move {
        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(async {
                rx.await.ok(); // Wait for the shutdown signal from the sender `tx`
                println!("Shutting down local server...");
            })
            .await // Run the server until shutdown is triggered or an error occurs
    };

    // Return the shutdown sender and the server future
    Ok((tx, server_future))
}

fn sleep(ms: u64) {
    thread::sleep(std::time::Duration::from_millis(ms));
}

fn capture_pdf(tab: &headless_chrome::Tab, dark: bool) -> Result<(), Box<dyn std::error::Error>> {
    let theme = if dark { "dark" } else { "light" };

    // Create PDF
    let pdf_data = tab.print_to_pdf(Some(PrintToPdfOptions {
        paper_width: Some(5.83 * 2.7),
        paper_height: Some(8.27 * 3.0),
        print_background: Some(true),
        margin_bottom: Some(0.0),
        margin_left: Some(0.0),
        margin_right: Some(0.0),
        margin_top: Some(0.0),
        ..PrintToPdfOptions::default()
    }))?;

    fs::write(format!("public/files/hmziqrs-{}-cv.pdf", theme), pdf_data)?;
    println!("{} pdf saved", theme);

    sleep(1000);

    Ok(())
}

fn get_content_height(tab: &headless_chrome::Tab) -> Result<f64, Box<dyn std::error::Error>> {
    tab.evaluate("window.toggleJpegs();", false)?;

    sleep(1000);

    // Toggle JPEG modez
    let height = tab
        .evaluate("document.body.offsetHeight", false)?
        .value
        .map(|v| v.as_f64())
        .unwrap()
        .unwrap();

    Ok(height)
}

fn capture_jpeg(tab: &headless_chrome::Tab, dark: bool) -> Result<(), Box<dyn std::error::Error>> {
    let theme = if dark { "dark" } else { "light" };

    // println!("bounds status {}", stat);

    let element = tab.find_element("html").unwrap();

    sleep(1000);

    // Take screenshot
    let screenshot = element
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg)
        .unwrap();

    // let screenshot =
    //     tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;

    fs::write(format!("public/files/hmziqrs-{}-cv.jpg", theme), screenshot)?;
    println!("{} jpeg saved", theme);

    sleep(1000);

    Ok(())
}
