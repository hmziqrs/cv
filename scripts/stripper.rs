use lightningcss::stylesheet::{ParserOptions, StyleSheet};
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

    let public_root = project_root
        .join("target")
        .join("dx")
        .join(package_name)
        .join("release")
        .join("web")
        .join("public");

    // Construct path to index.html
    let index_html_path = public_root.join("index.html");
    let assets_dir = public_root.join("assets");

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

            for element in document.select("script").unwrap() {
                let script_text = element.text_contents();
                println!(
                    "tototot : {} | {} \n\n\n",
                    script_text.contains("initial_dioxus_hydration_data"),
                    script_text
                );
                // Check if it's our initialization script
                if script_text.contains("// We can't use a module script")
                    && script_text.contains("import(")
                    && script_text.contains("wasm.main();")
                {
                    element.as_node().detach();
                }

                if script_text.contains("initial_dioxus_hydration_data") {
                    element.as_node().detach();
                }

                // Check for the hydration script
                if script_text.contains("hydrate_queue") || script_text.contains("dx_hydrate") {
                    element.as_node().detach();
                }
            }

            let modified_html = document.to_string();

            let hydration_node_regex =
                Regex::new(r#"\s*data-node-hydration=["']?\d+["']?"#).unwrap();
            let cleaned_html = hydration_node_regex
                .replace_all(&modified_html, "")
                .to_string();

            // Minify the HTML
            let minified = minify(cleaned_html.as_bytes(), &Cfg::default());

            // Write the minified HTML back to the file
            fs::write(&index_html_path, minified)?;

            println!("Successfully removed specified tags from index.html");
        }
        Err(e) => {
            return Err(format!(
                "Failed to read index.html at {}: {}",
                index_html_path.display(),
                e
            )
            .into());
        }
    }

    // for entry in fs::read_dir(assets_dir).unwrap() {
    //     let entry = entry?;
    //     let path = entry.path();
    //     if path.is_file() {
    //         let name = path.file_name().unwrap().to_string_lossy().to_string();

    //         if name.starts_with("tailwind") && name.ends_with(".css") {
    //             let css_content = fs::read_to_string(&path)?;

    //             let mut stylesheet = StyleSheet::parse(
    //                 &css_content,
    //                 ParserOptions {
    //                     filename: name,
    //                     ..ParserOptions::default()
    //                 },
    //             )
    //             .map_err(|e| format!("Failed to parse CSS: {:?}", e))
    //             .unwrap();

    //             stylesheet
    //                 .minify(lightningcss::stylesheet::MinifyOptions {
    //                     ..Default::default()
    //                 })
    //                 .map_err(|e| format!("Failed to minify CSS: {:?}", e))?;

    //             fs::write(path, serde_json::to_string(&stylesheet).unwrap());
    //         }
    //     }
    // }

    Ok(())
}
