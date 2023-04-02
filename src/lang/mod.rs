use std::str::FromStr;

#[derive(Debug)]
pub enum Lang {
    JavaScript,
    Rust,
    Go,
    Python,
    Php,
    Cpp,
}

impl FromStr for Lang {
    type Err = ();

    fn from_str(input: &str) -> Result<Lang, Self::Err> {
        match input {
            "JavaScript" => Ok(Lang::JavaScript),
            "javascript" => Ok(Lang::JavaScript),
            "TypeScript" => Ok(Lang::JavaScript),
            "typescript" => Ok(Lang::JavaScript),
            "js" => Ok(Lang::JavaScript),
            "ts" => Ok(Lang::JavaScript),
            "Rust" => Ok(Lang::Rust),
            "rust" => Ok(Lang::Rust),
            "rs" => Ok(Lang::Rust),
            "Go" => Ok(Lang::Go),
            "go" => Ok(Lang::Go),
            "Python" => Ok(Lang::Python),
            "python" => Ok(Lang::Python),
            "py" => Ok(Lang::Python),
            "Php" => Ok(Lang::Php),
            "php" => Ok(Lang::Php),
            "Cpp" => Ok(Lang::Cpp),
            "cpp" => Ok(Lang::Cpp),
            "c++" => Ok(Lang::Cpp),
            _ => Err(()),
        }
    }
}

impl Lang {
    pub fn get_langs_str() -> Vec<&'static str> {
        vec!["Javascript", "Go", "Rust", "Php", "Python", "Cpp"]
    }
}
