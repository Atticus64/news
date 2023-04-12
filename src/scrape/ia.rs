use json::object;
use std::process::exit;
use termimad::crossterm::style::Stylize;
use terminal_spinners::{SpinnerBuilder, EARTH};

use crate::{
    page::markdown::get_markdown_content,
    utils::config::{get_config, get_token_cohere, has_config},
};

pub fn get_ia_new_resume(link: String) -> Result<(), Box<dyn std::error::Error>> {
    let response = ureq::get(&link).call()?;
    let html = response.into_string()?;
    let markdown = get_markdown_content(&html);
    get_resume(&markdown)?;

    Ok(())
}

pub fn get_resume(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&EARTH)
        .text("Ia drafting novelty")
        .start();

    let json = object! {
      "model": "summarize-xlarge",
      "length": "long",
      "format": "bullets",
      "presence_penalty": 0,
      "text": text,
      "temperature": 0.3,
    };

    if has_config() {
        let config = get_config();
        let api_key: String = get_token_cohere(config);
        let authorization = format!("BAERER {api_key}");

        let resp = match ureq::post("https://api.cohere.ai/summarize")
            .set("Authorization", authorization.as_str())
            .set("Cohere-Version", "2021-11-08")
            .set("Content-Type", "application/json")
            .send_string(json.to_string().as_str())
        {
            Ok(r) => r,
            Err(e) => {
                handle.error();
                let message = e.to_string();
                if message.contains("401") {
                    let message =
                      "Failed to auth with cohere_token\nCheck your config $HOME/.config/news.json\nOr your token has expired";
                    println!("{}: {}", "Error".red(), message);
                } else {
                    let error_message = e.to_string();
                    println!("{}: {}", "Error".red(), error_message);
                }
                exit(1);
            }
        };

        let raw_json = resp.into_string()?;

        let data = json::parse(raw_json.as_str()).unwrap();

        let resume = data["summary"].to_string();

        println!("\n{resume}");

        handle.done();
    } else {
        handle.error();
        println!("Failed to get config cohere_token");
        println!(
            "Configure yout Cohere token in config file {}",
            "$HOME/.config/news.toml".yellow()
        );
        exit(1)
    }

    Ok(())
}
