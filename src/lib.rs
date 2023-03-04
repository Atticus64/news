use inquire::Select;
use page::markdown::View;
use page::markdown::{generate_view, get_markdonwwn_content};
use scrape::issues::get_js_issues_news;
use scrape::link::get_js_news;
use std::error::Error;
use std::str::FromStr;

pub mod page;
pub mod scrape;

pub async fn get_news() -> Result<(), Box<dyn Error>> {
    let options = vec!["Check Javascript News", "Exit"];

    let ans = Select::new("What's do you like to do?", options).prompt();

    match ans {
        Ok(choice) => match choice {
            "Exit" => std::process::exit(0),
            _ => println!("Js News"),
        },
        Err(_) => {
            println!("There was an error, please try again");
            std::process::exit(0);
        }
    }
    let (issues, issues_options) = get_js_issues_news().await?;

    let ans = Select::new("What new do you like to watch?", issues_options)
        .prompt()
        .expect("fail to read prompt news_issue user");

    let new_item = issues.iter().find(|new| new.title == ans);

    if let Some(value) = new_item {
        let (news, options) = get_js_news(value.link.as_str()).await?;
        let answer = Select::new("What new do you like to watch?", options)
            .prompt()
            .expect("fail to read prompt newsLink");

        let new_struct = news.iter().find(|new| new.title == answer);

        if let Some(new) = new_struct {
            let link = &new.link;
            let response = reqwest::get(link).await?;
            let url = response.url();

            // if is a video of youtube
            if url
                .domain()
                .expect("error not is a domain")
                .contains("youtube")
            {
                if webbrowser::open(new.link.as_str()).is_ok() {}
                return Ok(());
            }

            let view_select = Select::new("What view do you like to do?", vec!["Web", "Terminal"])
                .with_help_message("Enter the view of the new")
                .prompt()
                .expect("error reading prompt view select");

            let view = View::from_str(view_select).expect("failed to parse view");
            match view {
                View::Terminal => {
                    println!("{link}");

                    let html = response.text().await?;
                    let markdown = get_markdonwwn_content(&html);
                    if markdown.is_empty() {
                        println!("Content of new cannot be loaded in terminal");
                        println!("Opening browser instead");
                        if webbrowser::open(new.link.as_str()).is_ok() {}
                    } else {
                        generate_view(markdown.as_str())
                            .expect("failed to generate a markdown view");
                    }
                }
                View::Web => if webbrowser::open(new.link.as_str()).is_ok() {},
            }
        }
    }

    println!("res: {ans}");

    Ok(())
}
