use dioxus::{logger::tracing, prelude::*};

pub mod components;
mod metadata;
pub mod router;
pub mod screens;

fn main() {
    eprintln!("E PRINT MAIN");
    router::create_sitemap();

    // dioxus::launch(App);
    dioxus::LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            eprintln!("SERVER ONLY CFG");
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    IncrementalRendererConfig::new()
                        // Store static files in the public directory where other static assets like wasm are stored
                        .static_dir(router::static_dir())
                        // Don't clear the public folder on every build. The public folder has other files including the wasm
                        // binary and static assets required for the app to run
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        metadata::Metadata {}
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com"
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            "crossorigin": ""
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Geist+Mono:wght@400..600&family=Geist:wght@400..600&display=swap"
        }
        document::Link {
            rel: "stylesheet", href: asset!("/assets/tailwind.css")
        }
        document::Link {
            rel: "apple-touch-icon",
            sizes: "180x180",
            href: asset!("/public/fav/apple-touch-icon.png")
        }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            sizes: "32x32",
            href: asset!("/public/fav/favicon-32x32.png")
        }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            sizes: "16x16",
            href: asset!("/public/fav/favicon-16x16.png")
        }
        document::Link {
            rel: "manifest",
            href: asset!("/public/fav/site.webmanifest")
        }
        document::Script {
            src: asset!("/assets/script.js"),
            r#type: "text/javascript"
        }

        Router::<crate::router::Route> {}
    }
}
