//! # News
//!
//! `News` is a cli program to watch developer news from the terminal
use news_cli::get_news;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    match get_news() {
        Ok(_) => {},
        Err(_e) => {
           println!("Ocurrio un error")
        }
    }
    Ok(())
}
