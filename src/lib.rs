use colored::*;
use inquire::Select;
use lang::{get_lang, Lang};
use page::markdown::show_news;
use scrape::issues::{get_js_issues_news, get_rs_issues_news};
use scrape::link::{get_js_news, get_rs_news};
use std::error::Error;

pub mod lang;
pub mod page;
pub mod scrape;

pub async fn get_news() -> Result<(), Box<dyn Error>> {
    let options = vec!["Check Javascript News", "Check Rust News", "Exit"];
    println!("{} {}", "News".green(), "in terminal".blue());

    let ans = Select::new("What would you like to do?", options).prompt();

    let lang = match ans {
        Ok(choice) => match choice {
            "Exit" => std::process::exit(0),
            _ => get_lang(choice),
        },
        Err(_) => {
            println!("There was an error, please try again");
            std::process::exit(0);
        }
    };

    let (issues, issues_options) = match lang {
        Lang::Rust => get_rs_issues_news().await?,
        Lang::JavaScript => get_js_issues_news().await?,
    };

    // let (issues, issues_options) = get_js_issues_news().await?;

    let ans = Select::new("Which new would you like to watch?", issues_options)
        .prompt()
        .expect("fail to read prompt news_issue user");

    let new_item = issues.iter().find(|new| new.title == ans);

    if let Some(value) = new_item {
        let (news, options) = match lang {
            Lang::JavaScript => get_js_news(value.link.as_str()).await?,
            Lang::Rust => get_rs_news(value.link.as_str()).await?,
        };

        let answer = Select::new("What new do you like to watch?", options)
            .prompt()
            .expect("fail to read prompt newsLink");

        let new_struct = news.iter().find(|new| new.title == answer);

        if let Some(new) = new_struct {
            show_news(new).await?;
        }
    }

    println!("res: {ans}");

    Ok(())
}
