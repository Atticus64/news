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
            "Check Javascript News" => Ok(Lang::JavaScript),
            "Check Rust News" => Ok(Lang::Rust),
            _ => Err(()),
        }
    }
}

pub fn get_lang(lang: &str) -> Lang {
    Lang::from_str(lang).unwrap()
}
