//! # News
//!
//! `News` is a cli program to watch developer news from the terminal
use news::get_news;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    get_news().await?;
    Ok(())
}
