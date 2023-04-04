use clap::ArgMatches;
use inquire::Select;
use std::{error::Error, process::exit, str::FromStr};

use crate::{
    args::get_command,
    lang::Lang,
    scrape::{
        issues::{get_issues, get_latest_issue, Issue},
        link::{get_news_by_lang_and_resume, get_news_by_lang_and_show},
    },
    utils::{
        config::{get_config, get_langs_config, has_config},
        constants::VERSION,
    },
};

use super::select::{get_answer, get_answer_str};

pub fn lang_menu() -> Lang {
    let mut langs: Vec<String> = vec![];
    if has_config() {
        let content = get_config();
        langs = get_langs_config(content);
    }

    if langs.len() == 0 {
        let data = vec!["JavaScript", "Rust", "Go", "Python", "Php", "Cpp"];
        for l in data {
            let a = l.to_owned();
            langs.push(a);
        }
    }

    let language = get_answer(
        "Which language do you want to check news?",
        langs,
        "Lang no provided",
    );

    Lang::from_str(language.as_str()).expect("Fail to get lang from str")
}

pub async fn novelty_menu(
    issues: Vec<Issue>,
    options_issues: Vec<String>,
    lang: &Lang,
    ia_resumable: bool,
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

    if ia_resumable {
        get_news_by_lang_and_resume(lang, new).await?;
    } else {
        get_news_by_lang_and_show(lang, new).await?;
    }

    Ok(())
}

pub async fn all_news(args: ArgMatches) -> Result<(), Box<dyn Error>> {
    loop {
        let lang = lang_menu();

        let (issues, issues_options) = get_issues(&lang).await?;

        let ia_resumable = args.get_flag("resume");

        novelty_menu(issues, issues_options, &lang, ia_resumable).await?;

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
    if args.subcommand_matches("today").is_some() {
        if args.get_flag("resume") && args.contains_id("lang") {
            let lang_str = args.get_one::<String>("lang").expect("No string for lang");
            let lang = Lang::from_str(lang_str).unwrap();

            check_ultimate_news(Some(lang), true).await?;
            exit(0);
        } else if args.get_flag("resume") {
            check_ultimate_news(None, true).await?;
            exit(0);
        }

        check_ultimate_news(None, false).await?;
        exit(0);
    }

    if args.get_flag("resume") {
        check_ultimate_news(None, true).await?;
        exit(0);
    }

    if args.get_flag("vers") {
        println!("news {}", VERSION);
        exit(0);
    }

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

            get_news_by_lang_and_show(&lang, &issue).await?;

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
        // improve this code
        let langs = Lang::get_langs_str()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("The langs supported in the cli are: {langs}");
        exit(0)
    }

    let mut cmd = get_command();

    cmd.print_help()?;

    Ok(())
}

pub async fn check_ultimate_news(
    language: Option<Lang>,
    ai_resume: bool,
) -> Result<(), Box<dyn Error>> {
    if language.is_none() {
        loop {
            let lang = lang_menu();

            let issue = get_latest_issue(&lang).await?;

            if ai_resume {
                get_news_by_lang_and_resume(&lang, &issue).await?;
            } else {
                get_news_by_lang_and_show(&lang, &issue).await?;
            }

            let phrase = "Do you want to search more news?".to_string();

            let wants_research_lang = Select::new(&phrase, vec!["No", "Yes"])
                .with_help_message("Select Yes or No")
                .prompt()
                .unwrap_or("Cancel");

            if wants_research_lang == "Cancel" || wants_research_lang == "No" {
                break;
            }
        }
    } else {
        loop {
            let lang = language.as_ref().expect("Failed to as ref lang");
            let issue = get_latest_issue(lang).await?;

            if ai_resume {
                get_news_by_lang_and_resume(lang, &issue).await?;
            } else {
                get_news_by_lang_and_show(lang, &issue).await?;
            }

            let phrase = "Do you want to search more news?".to_string();

            let wants_research_lang = Select::new(&phrase, vec!["No", "Yes"])
                .with_help_message("Select Yes or No")
                .prompt()
                .unwrap_or("Cancel");

            if wants_research_lang == "Cancel" || wants_research_lang == "No" {
                break;
            }
        }
    }
    Ok(())
}
