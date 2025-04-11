use scraper::{Html, Selector};
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
            return None; // Reached the root directory without finding Cargo.toml
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Find and read Cargo.toml
    let cargo_toml_path = find_cargo_toml().ok_or("Could not find Cargo.toml file")?;
    let content = fs::read_to_string(&cargo_toml_path)?;
    let parsed_toml: cargo_toml::Value = content.parse()?;

    // Get package name
    let package_name = parsed_toml
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .ok_or("Failed to find package name in Cargo.toml")?;

    println!("Package name: {}", package_name);

    // Get the project root directory (parent of the directory containing Cargo.toml)
    let project_root = cargo_toml_path
        .parent()
        .ok_or("Failed to get parent directory of Cargo.toml")?;

    // Construct path to index.html
    let index_html_path = project_root
        .join("target")
        .join("dx")
        .join(package_name)
        .join("release")
        .join("web")
        .join("public")
        .join("index.html");

    // Read index.html
    match fs::read_to_string(&index_html_path) {
        Ok(html_content) => {
            use kuchikiki::traits::*;

            println!(
                "Successfully read index.html from: {}",
                index_html_path.display()
            );

            let document = kuchikiki::parse_html().one(html_content);

            for element in document
                .select("link[rel='preload'][as='script'][href^='/./assets/cv-']")
                .unwrap()
            {
                element.as_node().detach();
            }

            // Remove WASM preload link
            for element in document
                .select("link[rel='preload'][as='fetch'][type='application/wasm']")
                .unwrap()
            {
                element.as_node().detach();
            }

            let script_nodes: Vec<kuchikiki::NodeRef> = document
                .select("script")
                .unwrap()
                .map(|select| select.as_node().clone())
                .collect();

            for script_node in script_nodes {
                // Check if it's our initialization script
                let text_content = script_node.text_contents();
                if text_content.contains("// We can't use a module script")
                    && text_content.contains("import(")
                    && text_content.contains("wasm.main();")
                {
                    script_node.detach();
                }

                // Check for the hydration script
                if text_content.contains("window.hydrate_queue=[]")
                    && text_content.contains("window.dx_hydrate=")
                {
                    script_node.detach();
                }
            }

            let modified_html = document.to_string();

            // Write the modified HTML back to the file
            fs::write(&index_html_path, modified_html)?;

            println!("Successfully removed specified tags from index.html");

            Ok(())
        }
        Err(e) => Err(format!(
            "Failed to read index.html at {}: {}",
            index_html_path.display(),
            e
        )
        .into()),
    }
}
