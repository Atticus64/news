use clap::ArgMatches;
use inquire::Select;
use std::{error::Error, process::exit, str::FromStr};

use crate::{
    args::get_command,
    lang::Lang,
    scrape::{
        issues::{get_issues, get_latest_issue, Issue},
        link::get_news_by_lang,
    },
};

use super::select::{get_answer, get_answer_str};

pub fn lang_menu() -> Lang {
    let langs = vec!["JavaScript", "Rust", "Go", "Python", "Php", "Cpp"];
    let language = get_answer(
        "Which language do you want to check news?",
        langs,
        "Lang no provided",
    );

    Lang::from_str(language).expect("Fail to get lang from str")
}

pub async fn novelty_menu(
    issues: Vec<Issue>,
    options_issues: Vec<String>,
    lang: &Lang,
) -> Result<(), Box<dyn Error>> {
    let novelty = get_answer_str(
        "Which new would you like to watch?",
        options_issues,
        "no novelty provided",
    );

    let new_item = issues.iter().find(|new| new.title == novelty);

    let new = match new_item {
        Some(value) => value,
        None => {
            println!("Operation cancelled: New cancel");
            std::process::exit(1);
        }
    };

    get_news_by_lang(lang, new).await?;

    Ok(())
}

pub async fn all_news() -> Result<(), Box<dyn Error>> {
    loop {
        let lang = lang_menu();

        let (issues, issues_options) = get_issues(&lang).await?;

        novelty_menu(issues, issues_options, &lang).await?;

        let phrase = "Do you want to search more news?".to_string();

        let wants_research_lang = Select::new(&phrase, vec!["No", "Yes"])
            .with_help_message("Select Yes or No")
            .prompt()
            .unwrap_or("Cancel");

        if wants_research_lang == "Cancel" || wants_research_lang == "No" {
            break;
        }
    }

    Ok(())
}

pub async fn manage_news(args: ArgMatches) -> Result<(), Box<dyn Error>> {
    if args.contains_id("lang") {
        let lang_str = args.get_one::<String>("lang").expect("No string for lang");
        let lang = match Lang::from_str(lang_str) {
            Ok(value) => value,
            Err(_) => {
                println!("No exist lang {lang_str}");
                exit(1)
            }
        };

        loop {
            let issue = get_latest_issue(&lang).await?;

            get_news_by_lang(&lang, &issue).await?;

            let phrase = "Do you want to search more news?".to_string();

            let wants_research_lang = Select::new(&phrase, vec!["No", "Yes"])
                .with_help_message("Select Yes or No")
                .prompt()
                .unwrap_or("Cancel");

            if wants_research_lang == "Cancel" || wants_research_lang == "No" {
                break;
            }
        }
        exit(0)
    }

    if args.get_flag("list") {
        let langs = Lang::get_langs_str()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("The langs supported in the cli are: {langs}");
        exit(0)
    }

    if args.subcommand_matches("today").is_some() {
        check_ultimate_news().await?;
        exit(0);
    }

    let mut cmd = get_command();

    cmd.print_help()?;

    Ok(())
}

pub async fn check_ultimate_news() -> Result<(), Box<dyn Error>> {
    loop {
        let lang = lang_menu();
        let issue = get_latest_issue(&lang).await?;

        get_news_by_lang(&lang, &issue).await?;

        let phrase = "Do you want to search more news?".to_string();

        let wants_research_lang = Select::new(&phrase, vec!["No", "Yes"])
            .with_help_message("Select Yes or No")
            .prompt()
            .unwrap_or("Cancel");

        if wants_research_lang == "Cancel" || wants_research_lang == "No" {
            break;
        }
    }

    Ok(())
}
