use minify_html::{minify, Cfg};
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn find_cargo_toml() -> Option<PathBuf> {
    let mut current_dir = std::env::current_dir().ok()?;

    loop {
        let cargo_toml_path = current_dir.join("Cargo.toml");
        if cargo_toml_path.exists() {
            return Some(cargo_toml_path);
        }

        if !current_dir.pop() {
            return None;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cargo_toml_path = find_cargo_toml().ok_or("Could not find Cargo.toml file")?;
    let manifest = cargo_toml::Manifest::from_path(&cargo_toml_path)?;

    let package_name = &manifest.package().name;

    println!("Package name: {}", package_name);

    let project_root = cargo_toml_path
        .parent()
        .ok_or("Failed to get parent directory of Cargo.toml")?;

    let public_root = project_root
        .join("target")
        .join("dx")
        .join(package_name)
        .join("release")
        .join("web")
        .join("public");

    let index_html_path = public_root.join("index.html");

    match fs::read_to_string(&index_html_path) {
        Ok(html_content) => {
            use kuchikiki::traits::*;

            println!("Successfully read index.html");

            let sink = kuchikiki::parse_html().one(html_content);
            let document = sink.document_node.clone();

            for element in document
                .select("link[rel='preload'][as='script'][href^='/./assets/cv-']")
                .unwrap()
            {
                element.as_node().detach();
            }

            for element in document
                .select("link[rel='preload'][as='fetch'][type='application/wasm']")
                .unwrap()
            {
                element.as_node().detach();
            }

            let script_text = vec![
                "// We can't use a module script",
                "initial_dioxus_hydration_data",
                "hydrate_queue",
                "dx_hydrate",
            ];
            for text in script_text.iter() {
                for element in document.select("script").unwrap() {
                    if element.text_contents().contains(text) {
                        element.as_node().detach();
                    }
                }
            }

            let modified_html = document.to_string();

            let hydration_node_regex =
                Regex::new(r#"\s*data-node-hydration=["']?\d+["']?"#).unwrap();

            let cleaned_html = hydration_node_regex
                .replace_all(&modified_html, "")
                .to_string();

            let minified = minify(cleaned_html.as_bytes(), &Cfg::default());

            fs::write(&index_html_path, minified)?;

            println!("Successfully removed specified tags from index.html");
        }
        Err(e) => {
            return Err(format!(
                "Failed to read index.html {}: {}",
                index_html_path.display(),
                e
            )
            .into());
        }
    }

    Ok(())
}
