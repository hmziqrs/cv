use minify_html::{minify, Cfg};
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

            // let cfg = Cfg {
            //     // Allow more aggressive minification techniques
            //     allow_noncompliant_unquoted_attribute_values: true,
            //     allow_optimal_entities: true,
            //     allow_removing_spaces_between_attributes: true,

            //     // Remove optional elements to reduce size
            //     keep_closing_tags: false,
            //     keep_comments: false,
            //     keep_html_and_head_opening_tags: false,
            //     keep_input_type_text_attr: false,
            //     keep_ssi_comments: false,

            //     // Enable minification of embedded content
            //     minify_css: true,
            //     minify_js: true,
            //     minify_doctype: true,

            //     // Keep template syntax intact if needed
            //     preserve_brace_template_syntax: false, // Set to true if using Mustache/Handlebars
            //     preserve_chevron_percent_template_syntax: false, // Set to true if using EJS/ASP

            //     // Remove unnecessary elements
            //     remove_bangs: true,
            //     remove_processing_instructions: true,
            // };

            // // Minify the HTML
            // let minified = minify(modified_html.as_bytes(), &cfg);

            // Write the minified HTML back to the file
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
