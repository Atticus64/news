use std::str::FromStr;

#[derive(Debug)]
pub enum Lang {
    JavaScript,
    Rust,
}

impl FromStr for Lang {
    type Err = ();

    fn from_str(input: &str) -> Result<Lang, Self::Err> {
        match input {
            "JavaScript" => Ok(Lang::JavaScript),
            "Rust" => Ok(Lang::Rust),
            _ => Err(()),
        }
    }
}

pub fn get_lang(lang: &str) -> Lang {
    Lang::from_str(lang).unwrap()
}

pub fn get_lang_str(lang: Lang) -> String {
    let str = match lang {
        Lang::JavaScript => "JavaScript",
        Lang::Rust => "Rust",
    };

    String::from(str)
}
