use clap::ArgMatches;
use inquire::Select;
use colored::*;
use std::{error::Error, process::exit, str::FromStr};

use crate::{
    args::get_command,
    lang::Lang,
    scrape::{
        issues::{get_issues, get_latest_issue, Issue, get_last_issue},
        link::{get_news_by_lang_and_resume, get_news_by_lang_and_show, get_lang_news},
    },
    utils::constants::VERSION,
};

use super::select::get_answer;

pub fn lang_menu() -> Lang {
    let langs = vec![
        String::from("JavaScript"),
        String::from("Rust"),
        String::from("Go"),
        String::from("Python"),
        String::from("Php"),
        String::from("Cpp"),
    ];

    let language = get_answer(
        "Which language do you want to check news?",
        langs,
        "Lang no provided",
    );

    Lang::from_str(language.as_str()).expect("Fail to get lang from str")
}

pub fn novelty_menu(
    issues: Vec<Issue>,
    options_issues: Vec<String>,
    lang: &Lang,
    ia_resumable: bool,
) -> Result<(), Box<dyn Error>> {
    let novelty = get_answer(
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
        get_news_by_lang_and_resume(lang, new)?;
    } else {
        get_news_by_lang_and_show(lang, new)?;
    }

    Ok(())
}

pub fn all_news(args: ArgMatches) -> Result<(), Box<dyn Error>> {
    loop {
        let lang = lang_menu();

        let (issues, issues_options) = get_issues(&lang)?;

        let ia_resumable = args.get_flag("resume");

        novelty_menu(issues, issues_options, &lang, ia_resumable)?;

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

fn today_news(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
	if args.get_flag("resume") && args.contains_id("lang") {
		let lang_str = args.get_one::<String>("lang").expect("No string for lang");
		let lang = Lang::from_str(lang_str).unwrap();

		check_ultimate_news(Some(lang), true)?;
		exit(0);
	} else if args.get_flag("resume") {
			check_ultimate_news(None, true)?;
			exit(0);
	}

	check_ultimate_news(None, false)?;
	exit(0);
}

/// Manage cases of flags in app
pub fn manage_news(args: ArgMatches) -> Result<(), Box<dyn Error>> {
    if args.subcommand_matches("today").is_some() {
				today_news(&args)?;
    }

    if args.subcommand_matches("list").is_some() {
        let list = args.subcommand().unwrap();
        let value = list.1.get_one::<String>("language").unwrap();
        let lang = match Lang::from_str(value) {
            Ok(lang) => lang,
            Err(_) => {
                println!("Lang is not valid");
                exit(1);
            }
        };

        let issue = get_last_issue(&lang)?;

        let news_array = get_lang_news(&lang, &issue)?;

        for new in news_array {
            println!("{}",  new.title.blue());
            println!("{} {}", "->".bright_green(), new.link);
            println!();
        }

        exit(0);
    }

    if args.get_flag("resume") {
        check_ultimate_news(None, true)?;
        exit(0);
    }

    if args.get_flag("vers") {
        println!("news {}", VERSION);
        exit(0);
    }

    if args.contains_id("lang") {
        lang_news(&args)?;
    }

    if args.get_flag("support") {
        list_langs();
    }

    let mut cmd = get_command();

    cmd.print_help()?;

    Ok(())
}

pub fn lang_news(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let lang_str = args.get_one::<String>("lang").expect("No string for lang");
    let lang = match Lang::from_str(lang_str) {
        Ok(value) => value,
        Err(_) => {
            println!("No exist lang {lang_str}");
            exit(1)
        }
    };

    loop {
        let issue = get_latest_issue(&lang)?;

        get_news_by_lang_and_show(&lang, &issue)?;

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

pub fn list_langs() {
    let langs = Lang::get_langs_str()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("The langs supported in the cli are: {langs}");
    exit(0)
}

fn news_with_lang(lang: &Lang, ia_resumable: bool) -> Result<bool, Box<dyn Error>> {
	let issue = get_latest_issue(lang)?;

	if ia_resumable {
			get_news_by_lang_and_resume(lang, &issue)?;
	} else {
			get_news_by_lang_and_show(lang, &issue)?;
	}

	let phrase = "Do you want to search more news?".to_string();

	let wants_research_lang = Select::new(&phrase, vec!["No", "Yes"])
			.with_help_message("Select Yes or No")
			.prompt()
			.unwrap_or("Cancel");

	if wants_research_lang == "Cancel" || wants_research_lang == "No" {
			Ok(false)
	} else {
			Ok(true)
	}
}

pub fn check_ultimate_news(language: Option<Lang>, ai_resume: bool) -> Result<(), Box<dyn Error>> {
    if language.is_none() {
        loop {
            let lang = lang_menu();

						let research = news_with_lang(&lang, ai_resume)?;

						if !research {
							break
						}

        }
    } else {
        loop {
					let l = language.as_ref().unwrap();
          let research = news_with_lang(l, ai_resume)?;
					if !research {
						break
					}
          
        }
    }

    Ok(())
}
