use clap::{Arg, ArgMatches, Command};

use crate::utils::constants::VERSION;

pub fn get_command() -> Command {
    Command::new("news")
        // creamos las optiones de la app
        .version(VERSION)
        .author("Jonathan @Atticus64 in Github")
        .about("CLI to watch news in the terminal 🐢 developed by Jonathan")
        .arg(
            Arg::new("lang")
                .short('l')
                .long("lang")
                .required(false)
                .num_args(1)
                .value_names(["Language"])
                .help("Choose a lang for the news"),
        )
        .arg(
            Arg::new("vers")
                .short('v')
                .required(false)
                .num_args(0)
                .help("Show news version"),
        )
        .arg(
            Arg::new("support")
                .long("sup")
                .required(false)
                .num_args(0)
                .help("Show lang supported"),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .short('a')
                .num_args(0)
                .required(false)
                .help("Check all news in the history of langs"),
        )
        .arg(
            Arg::new("resume")
                .long("resume")
                .short('r')
                .num_args(0)
                .required(false)
                .help("Resume the new selected by IA technology"),
        )
        .subcommand(
            Command::new("list")
            .about("list latest news of a lang")
            .arg(
                Arg::new("language")
                .required(true)
                .help("Language to search news"),
                )
            )
        .subcommand(
            Command::new("today")
                .about("Check the latest news")
                .arg(
                    Arg::new("lang")
                        .short('l')
                        .required(false)
                        .long("lang")
                        .help("Lang of the news"),
                )
                .arg(
                    Arg::new("resume")
                        .short('r')
                        .num_args(0)
                        .required(false)
                        .help("Resume the new selected by IA technology"),
                ),
        )
}

/// Get ArgMatches of the app
pub fn get_args() -> ArgMatches {
    let command = get_command();

    command.get_matches()
}
