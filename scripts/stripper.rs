use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

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
    let cargo_toml_path = find_cargo_toml().ok_or("Could not find Cargo.toml file")?;

    let content = fs::read_to_string(cargo_toml_path)?;
    let parsed_toml: cargo_toml::Value = content.parse()?;

    if let Some(package) = parsed_toml.get("package") {
        if let Some(name) = package.get("name") {
            if let Some(name_str) = name.as_str() {
                println!("Package name: {}", name_str);
                return Ok(());
            }
        }
    }

    Err("Failed to find package name in Cargo.toml".into())
}
