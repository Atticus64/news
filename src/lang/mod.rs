use std::str::FromStr;

#[derive(Debug)]
pub enum Lang {
    JavaScript,
    Rust,
    Go,
    Python,
}

impl FromStr for Lang {
    type Err = ();

    fn from_str(input: &str) -> Result<Lang, Self::Err> {
        match input {
            "JavaScript" => Ok(Lang::JavaScript),
            "Rust" => Ok(Lang::Rust),
            "Go" => Ok(Lang::Go),
            "Python" => Ok(Lang::Python),
            _ => Err(()),
        }
    }
}

pub fn get_lang(lang: &str) -> Lang {
    Lang::from_str(lang).expect("Failed fo get lang from &str")
}

pub fn get_lang_str(lang: Lang) -> String {
    let str = match lang {
        Lang::JavaScript => "JavaScript",
        Lang::Rust => "Rust",
        Lang::Go => "Go",
        Lang::Python => "Python",
    };

    String::from(str)
}
