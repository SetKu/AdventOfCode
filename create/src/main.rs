use std::env::{args, current_dir};
use std::fs::{read_dir, read_to_string, write};
use std::process::exit;
use std::process::Command;

fn main() {
    // VALIDATION

    let run_dir = current_dir().expect("Couldn't access process directory.");
    let contents = read_dir(run_dir.clone()).expect("Failed to read current directory.");
    let has_crate_source = contents.into_iter().any(|entry| {
        if let Ok(unwrapped) = entry {
            let path = unwrapped.path();

            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    return name == env!("CARGO_CRATE_NAME"); // "create"
                }
            }
        }

        false
    });

    if !has_crate_source {
        println!("This binary should be run just above its crate directory. Ensure \"{}\" is a child of the working path.", env!("CARGO_CRATE_NAME"));
        exit(1);
    }

    // CRATE CREATION

    let arg = args()
        .skip(1)
        .next()
        .expect("Expected a name for the binary as an argument.");
    // let name = "AoC-".to_string() + &arg;
    let name = arg;

    Command::new("cargo")
        .args(["new", "--bin", &name])
        .output()
        .unwrap();

    println!("Created new crate in {}", run_dir.to_str().unwrap());

    // UPDATE MAIN.RS

    let mut main_path = run_dir.clone();
    main_path.push(&name);
    main_path.push("src/main.rs");
    let main_path_str = main_path.to_str().unwrap();

    let mut main_text = read_to_string(main_path_str).unwrap();
    main_text.insert_str(0, "#![allow(non_snake_case)]\n\n");
    write(main_path_str, main_text).expect("Failed to save source file.");

    // UPDATE WORKSPACE CONFIG

    let mut config_path = run_dir;
    config_path.push("Cargo.toml");
    let config_str = read_to_string(config_path.clone()).expect("Failed to read workspace config.");
    let mut workspace_config =
        toml::from_str::<toml::Value>(&config_str).expect("Failed to parse workspace config.");

    assert!(
        workspace_config.is_table(),
        "The contents of the config file is strange..." // appropriate error :)
    );

    workspace_config
        .get_mut("workspace")
        .unwrap()
        .get_mut("members")
        .unwrap()
        .as_array_mut()
        .unwrap()
        .push(toml::Value::String(name));

    let new_config_str = toml::to_string_pretty(&workspace_config)
        .expect("Failed to convert memory representation of Config.toml back to a string.");
    write(config_path, new_config_str).expect("Failed to write back to Config.toml");
}
