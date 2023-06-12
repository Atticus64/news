pub mod cpp;
pub mod go;
pub mod javascript;
pub mod php;
pub mod python;
pub mod rust;

use std::error::Error;

use crate::lang::Lang;

use self::{
    cpp::{get_cpp_issues_news, get_latest_cpp_issue, get_last_cpp_issue},
    go::{get_go_issues_news, get_latest_go_issue, get_last_go_issue},
    javascript::{get_js_issues_news, get_latest_js_issue, get_last_js_issue},
    php::{get_latest_php_issue, get_php_issues_news, get_last_php_issue},
    python::{get_latest_py_issue, get_py_issues_news, get_last_py_issue},
    rust::{get_latest_rs_issue, get_rs_issues_news, get_last_rs_issue},
};

#[derive(Debug, Clone)]
pub struct Issue {
    pub title: String,
    pub link: String,
}

pub  fn get_issues(lang: &Lang) -> Result<(Vec<Issue>, Vec<String>), Box<dyn Error>> {
    let (issues, issues_options) = match lang {
        Lang::JavaScript => get_js_issues_news()?,
        Lang::Rust => get_rs_issues_news()?,
        Lang::Go => get_go_issues_news()?,
        Lang::Python => get_py_issues_news()?,
        Lang::Php => get_php_issues_news()?,
        Lang::Cpp => get_cpp_issues_news()?,
    };

    Ok((issues, issues_options))
}

pub  fn get_latest_issue(lang: &Lang) -> Result<Issue, Box<dyn Error>> {
    Ok(match lang {
        Lang::JavaScript => get_latest_js_issue()?,
        Lang::Rust => get_latest_rs_issue()?,
        Lang::Go => get_latest_go_issue()?,
        Lang::Python => get_latest_py_issue()?,
        Lang::Php => get_latest_php_issue()?,
        Lang::Cpp => get_latest_cpp_issue()?,
    })
}

pub fn get_last_issue(lang: &Lang) -> Result<Issue, Box<dyn Error>> {
     Ok(match lang {
        Lang::JavaScript => get_last_js_issue()?,
        Lang::Rust => get_last_rs_issue()?,
        Lang::Go => get_last_go_issue()?,
        Lang::Python => get_last_py_issue()?,
        Lang::Php => get_last_php_issue()?,
        Lang::Cpp => get_last_cpp_issue()?,
    })
}
