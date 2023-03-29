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
            Arg::new("list")
                .long("list")
                .num_args(0)
                .required(false)
                .help("List programming languages available for news"),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .short('a')
                .num_args(0)
                .required(false)
                .help("Check all news in the history of langs"),
        )
        .subcommand(
            Command::new("today").about("Check the latest news").arg(
                Arg::new("lang")
                    .short('l')
                    .long("lang")
                    .help("Lang of the news"),
            ),
        )
}

pub fn get_args() -> ArgMatches {
    let command = get_command();

    command.get_matches()
}
