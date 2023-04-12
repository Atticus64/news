use dirs;
use json;
use std::fs;

const CONFIG_PATH: &str = ".config/news.json";

// pub fn get_langs_config(content: String) -> Vec<String> {
pub fn get_token_cohere(content: String) -> String {
    let json = json::parse(content.as_str()).unwrap();
    let token = &json["cohere_token"].to_string();

    token.to_string()
}

fn get_home() -> String {
    let buf = dirs::home_dir().expect("Failed to get home path");
    buf.to_str()
        .expect("Failed to converto to string")
        .to_string()
}

pub fn has_config() -> bool {
    let home = get_home();
    let path = format!("{home}/{}", CONFIG_PATH);

    std::path::Path::new(&path).exists()
}

pub fn get_config() -> String {
    let home = get_home();
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
