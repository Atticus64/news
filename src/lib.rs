use args::get_args;
use std::error::Error;
use tui::menu::{all_news, manage_news};

mod args;
mod lang;
mod page;
pub mod scrape;
mod tui;
mod utils;

pub fn get_news() -> Result<(), Box<dyn Error>> {
    let args = get_args();

    let show_all = args.get_flag("all");

    if show_all {
        all_news(args)?;
    } else {
        manage_news(args)?;
    }

    Ok(())
}
