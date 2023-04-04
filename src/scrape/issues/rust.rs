use crabquery::Document;
use std::error::Error;
use terminal_spinners::{SpinnerBuilder, FLIP, MOON};

use super::Issue;

use reqwest::blocking;

pub  fn get_rs_issues_news() -> Result<(Vec<Issue>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&FLIP)
        .text(" Fetching Rust Issues")
        .start();
    const RUST_WEEKLY_URL: &str = "https://this-week-in-rust.org/blog/archives/index.html";

    let response = blocking::get(RUST_WEEKLY_URL)?;

    let text = response.text()?;

    let doc = Document::from(text);

    let issues = doc.select(".post-title");

    let mut vec_issues: Vec<Issue> = vec![];
    for issue in issues {
        let date_raw = issue
            .select(".time-prefix")
            .first()
            .expect("failed to get first element")
            .children()
            .first()
            .expect("failed to get first element children date issues rs")
            .text()
            .expect("Failed to converto to String");
        let title_raw = issue
            .select(".text-right")
            .first()
            .expect("Failed to get first element")
            .children()
            .first()
            .expect("Failed to get first element children title issues rs")
            .text()
            .expect("Failed to convert to String");
        let link = issue
            .select(".text-right")
            .first()
            .expect("Failed to get first element")
            .children()
            .first()
            .expect("Failed to get first element")
            .attr("href")
            .expect("Failed to get attr element");
        let title = format!("{title_raw} - {date_raw}");
        let new = Issue { title, link };
        vec_issues.push(new);
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_issues, issues_options))
}

pub  fn get_latest_rs_issue() -> Result<Issue, Box<dyn Error>> {
    const RUST_WEEKLY_URL: &str = "https://this-week-in-rust.org/blog/archives/index.html";

    let handle = SpinnerBuilder::new()
        .spinner(&MOON)
        .text("Fetching Rust Last Issue")
        .start();

    let response = blocking::get(RUST_WEEKLY_URL)?;

    let text = response.text()?;

    let doc = Document::from(text);

    let issues = doc.select(".post-title");

    let first = issues.first().expect("Failed to get first issue");

    let date_raw = first
        .select(".time-prefix")
        .first()
        .expect("failed to get first element")
        .children()
        .first()
        .expect("failed to get first element children date issues rs")
        .text()
        .expect("Failed to converto to String");
    let title_raw = first
        .select(".text-right")
        .first()
        .expect("Failed to get first element")
        .children()
        .first()
        .expect("Failed to get first element children title issues rs")
        .text()
        .expect("Failed to convert to String");

    let link = first
        .select(".text-right")
        .first()
        .expect("Failed to get first element")
        .children()
        .first()
        .expect("Failed to get first element")
        .attr("href")
        .expect("Failed to get attr element");

    let title = format!("{title_raw} - {date_raw}");

    let new = Issue { title, link };

    handle.done();

    Ok(new)
}
