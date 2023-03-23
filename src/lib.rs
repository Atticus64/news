use crate::lang::get_lang_str;
use crate::scrape::issues::{get_go_issues_news, get_py_issues_news};
use crate::scrape::link::{get_go_news, get_py_news};
use args::get_args;
use colored::*;
use inquire::Select;
use lang::Lang;
use page::markdown::show_news;
use scrape::issues::{get_js_issues_news, get_rs_issues_news};
use scrape::link::{get_js_news, get_rs_news};
use std::error::Error;
use std::process::exit;
use std::str::FromStr;

pub mod args;
pub mod lang;
pub mod page;
pub mod scrape;
pub mod utils;

pub async fn get_news() -> Result<(), Box<dyn Error>> {
    let _args = get_args();
    let options = vec!["Check News", "Exit"];
    println!("{} {}", "News".green(), "in terminal".blue());

    let ans = Select::new("What would you like to do?", options).prompt();
    match ans {
        Ok(choice) => match choice {
            "Exit" => std::process::exit(0),
            _ => (),
        },
        Err(_) => {
            println!("There was an error, please try again");
            std::process::exit(0);
        }
    };

    let langs = vec!["JavaScript", "Rust", "Go", "Python"];
    let language = match Select::new("which language do you want to check news?", langs).prompt() {
        Ok(lang) => lang,
        Err(_) => "Cancel",
    };

    if language == "Cancel" {
        manage_exit("Error lang no provided, operation cancelled")
    }

    loop {
        let lang = Lang::from_str(language).expect("Fail in convert language to type Lang");
        let (issues, issues_options) = match lang {
            Lang::JavaScript => get_js_issues_news().await?,
            Lang::Rust => get_rs_issues_news().await?,
            Lang::Go => get_go_issues_news().await?,
            Lang::Python => get_py_issues_news().await?,
        };

        let novelty =
            match Select::new("Which new would you like to watch?", issues_options).prompt() {
                Ok(new) => new,
                Err(_) => "Cancel".to_string(),
            };

        if novelty.as_str() == "Cancel" {
            manage_exit("fail to read prompt news_issue user")
        }

        let new_item = issues.iter().find(|new| new.title == novelty);
        if let Some(value) = new_item {
            let (news, options) = match lang {
                Lang::JavaScript => get_js_news(value.link.as_str()).await?,
                Lang::Rust => get_rs_news(value.link.as_str()).await?,
                Lang::Go => get_go_news(value.link.as_str()).await?,
                Lang::Python => get_py_news(value.link.as_str()).await?,
            };

            let answer = match Select::new("What new do you like to watch?", options).prompt() {
                Ok(ans) => ans,
                Err(_) => "Cancel".to_string(),
            };

            if answer.as_str() == "Cancel" {
                manage_exit("fail to read prompt newsLink")
            }

            let new_struct = news.iter().find(|new| new.title == answer);

            if let Some(new) = new_struct {
                show_news(new).await?;
            }

            let lang_str = get_lang_str(lang);
            let phrase = format!("Do you want to watch more news of the lang {lang_str}");

            let wants_research_lang = match Select::new(&phrase, vec!["No", "Yes"])
                .with_help_message("Select Yes or No")
                .prompt()
            {
                Ok(ans) => ans,
                Err(_) => "Cancel",
            };

            if wants_research_lang == "Cancel" || wants_research_lang == "No" {
                break;
            }
        }
    }

    Ok(())
}

fn manage_exit(err: &str) -> () {
    println!("Operation cancelled: {err}");
    exit(1);
}
