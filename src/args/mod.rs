use clap::{ArgMatches, Command};

pub fn get_args() -> ArgMatches {
    let opts = Command::new("news")
        // creamos las optiones de la app
        .version("0.1.0")
        .author("Jonathan @Atticus64 in Github")
        .about("CLI to watch news in the terminal 🐢")
        .get_matches();

    return opts;
}
