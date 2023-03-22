use clap::{ArgMatches, Command};

use crate::utils::constants::VERSION;

pub fn get_args() -> ArgMatches {
    let opts = Command::new("news")
        // creamos las optiones de la app
        .version(VERSION)
        .author("Jonathan @Atticus64 in Github")
        .about("CLI to watch news in the terminal 🐢")
        .get_matches();

    return opts;
}
