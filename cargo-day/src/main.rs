use std::env;
use std::fs;
use std::path::Path;
use std::process;
use toml_edit::{Array, DocumentMut, Item};

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_num = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: cargo day <number>");
        process::exit(1);
    };

    let day: u32 = match day_num.parse() {
        Ok(n) if (1..=25).contains(&n) => n,
        _ => {
            eprintln!("Day must be a number between 1 and 25");
            process::exit(1);
        }
    };

    let crate_name = format!("day{:02}", day);

    if Path::new(&crate_name).exists() {
        eprintln!("Crate '{}' already exists!", crate_name);
        process::exit(1);
    }

    println!("Creating crate: {}", crate_name);

    // Create the crate directory structure
    fs::create_dir(&crate_name).expect("Failed to create crate directory");
    fs::create_dir(format!("{}/src", crate_name)).expect("Failed to create src directory");

    // Create Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
"#,
        crate_name
    );
    fs::write(format!("{}/Cargo.toml", crate_name), cargo_toml)
        .expect("Failed to write Cargo.toml");

    // Create main.rs with template
    let main_rs = format!(
        r##"fn part1(input: &str) -> u64 {{
    // TODO: Implement part 1
    0
}}

fn part2(input: &str) -> u64 {{
    // TODO: Implement part 2
    0
}}

fn main() {{
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/{}.dat"));
    println!("Part 1: {{}}", part1(input));
    println!("Part 2: {{}}", part2(input));
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn test_part1() {{
        assert_eq!(part1(EXAMPLE), 0);
    }}

    #[test]
    fn test_part2() {{
        assert_eq!(part2(EXAMPLE), 0);
    }}
}}
"##,
        crate_name
    );
    fs::write(format!("{}/src/main.rs", crate_name), main_rs).expect("Failed to write main.rs");

    // Create empty data file
    fs::write(format!("data/{}.dat", crate_name), "").expect("Failed to write data file");

    println!("✓ Created {}/", crate_name);
    println!("✓ Created {}/src/main.rs", crate_name);
    println!("✓ Created data/{}.dat", crate_name);

    // Update workspace Cargo.toml
    update_workspace_toml(&crate_name);
}

fn update_workspace_toml(crate_name: &str) {
    let workspace_toml_path = "Cargo.toml";

    if !Path::new(workspace_toml_path).exists() {
        eprintln!("Warning: Cargo.toml not found in current directory. Skipping workspace update.");
        return;
    }

    let toml_content = fs::read_to_string(workspace_toml_path).expect("Failed to read Cargo.toml");

    let mut doc = toml_content
        .parse::<DocumentMut>()
        .expect("Failed to parse Cargo.toml");

    // Get or create workspace.members array
    if !doc.contains_key("workspace") {
        eprintln!(
            "Warning: No [workspace] section found in Cargo.toml. Skipping workspace update."
        );
        return;
    }

    let workspace = doc["workspace"]
        .as_table_mut()
        .expect("workspace should be a table");

    if !workspace.contains_key("members") {
        workspace["members"] = Item::Value(toml_edit::Value::Array(Array::new()));
    }

    let members = workspace["members"]
        .as_array_mut()
        .expect("workspace.members should be an array");

    // Check if crate already exists in members
    let crate_exists = members.iter().any(|item| item.as_str() == Some(crate_name));

    if !crate_exists {
        members.push(crate_name);

        fs::write(workspace_toml_path, doc.to_string())
            .expect("Failed to write updated Cargo.toml");

        println!("✓ Added '{}' to workspace Cargo.toml", crate_name);
    } else {
        println!("Note: '{}' already exists in workspace members", crate_name);
    }
}
