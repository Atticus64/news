use serde_derive::{Deserialize, Serialize};
use std::{fs, process::exit};
use termimad::crossterm::style::Stylize;
use toml;

const CONFIG_PATH: &str = "/.config/news.toml";

#[derive(Serialize, Deserialize, Debug)]
struct TokenConfig {
    cohere_token: String,
}

#[derive(Serialize, Deserialize, Debug)]

struct ConfigLangs {
    langs: Vec<String>,
}

// pub fn get_langs_config(content: String) -> Vec<String> {
pub fn get_langs_config(content: String) -> Vec<String> {
    let config: ConfigLangs = match toml::from_str(&content) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => ConfigLangs { langs: Vec::new() },
    };

    config.langs
}

pub fn get_token_cohere(content: String) -> String {
    let config: TokenConfig = match toml::from_str(&content) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            println!("\n{}: Failed to get config cohere_token", "Error".red());
            println!(
                "Configure your Cohere token in config file {}",
                "$HOME/.config/news.toml".yellow()
            );
            exit(1)
        }
    };

    config.cohere_token
}

pub fn has_config() -> bool {
    let home = std::env::var("HOME").unwrap();
    let path = format!("{home}/{}", CONFIG_PATH);

    std::path::Path::new(&path).exists()
}

pub fn get_config() -> String {
    let home = std::env::var("HOME").unwrap();
    let path = format!("{home}/{}", CONFIG_PATH);
    let data = fs::read_to_string(path);

    let content = match data {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Failed to read config file: {}", err);
            std::process::exit(1);
        }
    };

    content
}
