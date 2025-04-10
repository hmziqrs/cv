use dioxus::prelude::*;

#[component]
pub fn Metadata() -> Element {
    let json_ld = serde_json::json!({
        "@context": "https://schema.org",
        "@type": "Person",
        "name": "Hamza Iqbal",
        "jobTitle": "Full Stack Engineer",
        "description": "Full Stack Engineer specializing in Rust, Flutter, React Native, and web technologies.",
        "url": "https://hmziq.rs",
        "sameAs": [
            "https://github.com/hmziqrs",
            "https://linkedin.com/in/hmziqrs",
            "https://twitter.com/hmziqrs",
        ],
        "image": "https://cv.hmziq.rs/assets/pfp.jpg",
        "email": "hmziqrs@gmail.com",
    });

    let json_ld_string = serde_json::to_string(&json_ld).unwrap_or_default();

    rsx! {
        document::Title { "Hamza Iqbal - Full Stack Engineer | Rust, Flutter, React Native Developer" }
        document::Meta { name: "description", content: "Full Stack Engineer with 7+ years of experience in Rust, Flutter, React Native, NextJS, and NodeJS. View my portfolio, projects, and professional experience." }

        // Keywords
        document::Meta {
            name: "keywords",
            content: "Full Stack Engineer, Software Developer, Rust Developer, Flutter Developer, React Native Developer, NextJS Developer, NodeJS Developer, Mobile App Development, Web Development"
        }

        // Author
        document::Meta { name: "author", content: "Hamza Iqbal" }
        document::Link { rel: "author", href: "https://hmziq.rs" }

        // Open Graph Tags
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:locale", content: "en_US" }
        document::Meta { property: "og:url", content: "https://hmziq.rs" }
        document::Meta { property: "og:site_name", content: "Hamza Iqbal - Full Stack Engineer" }
        document::Meta { property: "og:title", content: "Hamza Iqbal - Full Stack Engineer Portfolio" }
        document::Meta { property: "og:description", content: "Full Stack Engineer specializing in Rust, Flutter, React Native, and web technologies." }
        document::Meta { property: "og:image", content: "/assets/pfp.png" }
        document::Meta { property: "og:image:width", content: "700" }
        document::Meta { property: "og:image:height", content: "700" }
        document::Meta { property: "og:image:alt", content: "Hamza Iqbal - Full Stack Engineer" }

        // Twitter Cards
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:site", content: "@hmziqrs" }
        document::Meta { name: "twitter:creator", content: "@hmziqrs" }
        document::Meta { name: "twitter:title", content: "Hamza Iqbal - Full Stack Engineer" }
        document::Meta { name: "twitter:description", content: "Full Stack Engineer specializing in Rust, Flutter, React Native, and web technologies." }
        document::Meta { name: "twitter:image", content: "/assets/pfp.png" }

        // Robots
        document::Meta { name: "robots", content: "index, follow" }
        document::Meta { name: "googlebot", content: "index, follow, max-video-preview:-1, max-image-preview:large, max-snippet:-1" }
        document::Script {
            r#type: "application/ld+json",
            dangerous_inner_html: "{json_ld_string}"
        }
    }
}
