use crate::page::render::generate_view;

use crate::scrape::ia::get_resume;
use crate::scrape::link::NewsLink;
use crate::tui::select::manage_exit;
use inquire::Select;
use nanohtml2text::html2text;
use std::str::FromStr;

use super::view::View;

pub fn get_markdown_content(html: &str) -> String {
    html2text(html)
}

/// render news in the terminal std output
pub fn show_news(new: &NewsLink) -> Result<(), Box<dyn std::error::Error>> {
    let link = &new.link;

    let response = ureq::get(link).call()?;

    let url = response.get_url();

    // if is a video of youtube
    if url.contains("youtube") {
        if webbrowser::open(new.link.as_str()).is_ok() {}
        return Ok(());
    }

    let view_select = Select::new(
        "What view do you like to do?",
        vec!["Web", "Terminal", "Ia Draft"],
    )
    .with_help_message("Enter the view of the new")
    .prompt()
    .unwrap_or("Cancel");

    if view_select == "Cancel" {
        manage_exit("No view provided")
    }

    let view = View::from_str(view_select).expect("failed to parse view");
    match view {
        View::Terminal => {
            let html = response.into_string()?;
            let markdown = get_markdown_content(&html);

            if markdown.is_empty() {
                println!("Content of new cannot be loaded in terminal");
                println!("Opening browser instead");
                if webbrowser::open(new.link.as_str()).is_ok() {}
            } else {
                generate_view(markdown.as_str()).expect("failed to generate a markdown view");
            }
        }
        View::Web => {
            webbrowser::open(new.link.as_str())?;
        }
        View::Ia => {
            let html = response.into_string()?;
            let markdown = get_markdown_content(&html);
            get_resume(&markdown)?;
        }
    }

    println!("Novelty link: {}", new.link);
    Ok(())
}
