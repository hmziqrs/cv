use dioxus::{logger::tracing, prelude::*};

use crate::screens::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    HomeScreen {},
}

#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}

pub fn static_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("public")
}

pub fn create_sitemap() {
    eprintln!("create_sitemap root start");

    #[cfg(not(debug_assertions))]
    server_only! {
        eprintln!("create_sitemap server_only");
        use std::io::Write;
        let domain = "https://cv.hmziq.rs";
        // Write a sitemap file on the server
        // The sitemap helps with SEO because google will deprioritize pages it finds that are not in the sitemap
        let all_routes = Route::static_routes();
        eprintln!("create_sitemap allroutes; {:?}", all_routes);
        _ = std::fs::create_dir_all(static_dir());
        let output_path = static_dir().join("sitemap.xml");
        let Ok(file) = std::fs::File::create(output_path) else {
            eprintln!("Failed to create sitemap file");
            return;
        };
        let mut writer = std::io::BufWriter::new(file);
        _ = writeln!(writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        _ = writeln!(writer, r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
        for route in all_routes {
            _ = writeln!(writer, r#"<url>"#);
            let url = format!("{}{}",domain, route);
            let escaped_url = askama_escape::escape(&url, askama_escape::Html);
            _ = writeln!(writer, r#"    <loc>{}</loc>"#, escaped_url);
            _ = writeln!(writer, r#"</url>"#);
        }
        _ = writeln!(writer, r#"</urlset>"#);



        let robots_content = format!(
            "User-Agent: *\nAllow: /\nSitemap: {}/sitemap.xml", // Use \n for newlines
            domain
        );

        _ = std::fs::write(static_dir().join("robots.txt"), robots_content);
    }
}
