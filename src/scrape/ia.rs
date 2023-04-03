use std::process::exit;

use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use termimad::crossterm::style::Stylize;
use terminal_spinners::{SpinnerBuilder, EARTH};

use crate::{
    page::markdown::get_markdown_content,
    utils::config::{get_config, get_token_cohere, has_config},
};

#[derive(Serialize, Deserialize)]
struct IaResponse {
    id: String,
    summary: String,
}

pub async fn get_ia_new_resume(link: String) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(link).await?;
    let html = response.text().await?;
    let markdown = get_markdown_content(&html);
    get_resume(&markdown).await?;

    Ok(())
}

pub async fn get_resume(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&EARTH)
        .text("Ia drafting novelty")
        .start();

    let client = reqwest::Client::new();
    let json = json!({
      "model": "summarize-xlarge",
      "length": "long",
      "format": "bullets",
      "presence_penalty": 0,
      "text": text,
      "temperature": 0.3,
    });

    let body = json.to_string();

    if has_config() {
        let config = get_config();
        let api_key: String = get_token_cohere(config);
        let authorization = format!("BAERER {api_key}");
        let resp = client
            .post("https://api.cohere.ai/summarize")
            .header("Authorization", authorization.as_str())
            .header("Cohere-Version", "2021-11-08")
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        let raw_json = resp.text().await?;

        let data: IaResponse = serde_json::from_str(&raw_json).unwrap();

        let resume = data.summary;

        handle.done();

        println!("{}", resume);
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
