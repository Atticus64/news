use std::error::Error;

use crabquery::Document;
use terminal_spinners::{SpinnerBuilder, FLIP, MOON};

use crate::scrape::link::get_html;

use super::Issue;

const PYTHON_WEEKLY_URL: &str = "https://pycoders.com/issues";

pub fn get_py_issues_news() -> Result<(Vec<Issue>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&FLIP)
        .text(" Fetching Python Issues")
        .start();

    let text = get_html(PYTHON_WEEKLY_URL);

    let doc = Document::from(text);

    let issues = doc.select(".my-1");

    let mut vec_issues: Vec<Issue> = vec![];
    for issue in issues {
        let title = issue
            .children()
            .first()
            .expect("Failed to get first children title rust ")
            .text()
            .expect("Failed to convert title rust to String");

        let link = issue
            .children()
            .first()
            .expect("Failed to get first element")
            .attr("href")
            .expect("Failed to get attr link rust issue");

        let issue_num = link
            .split('/')
            .last()
            .expect("Failed to get las item issues");

        let source = format!("{PYTHON_WEEKLY_URL}/{issue_num}");

        let new = Issue {
            title,
            link: source,
        };
        vec_issues.push(new);
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_issues, issues_options))
}

pub fn get_latest_py_issue() -> Result<Issue, Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&MOON)
        .text("Fetching Python Last Issue")
        .start();

    let text = get_html(PYTHON_WEEKLY_URL);

    let doc = Document::from(text);

    let issues = doc.select(".my-1");

    let issue = issues.first().expect("Failed to get first issue");

    let title = issue
        .children()
        .first()
        .expect("Failed to get first children title rust ")
        .text()
        .expect("Failed to convert title rust to String");

    let link = issue
        .children()
        .first()
        .expect("Failed to get first element")
        .attr("href")
        .expect("Failed to get attr link rust issue");

    let issue_num = link
        .split('/')
        .last()
        .expect("Failed to get las item issues");

    let source = format!("{PYTHON_WEEKLY_URL}/{issue_num}");

    let new = Issue {
        title,
        link: source,
    };

    handle.done();

    Ok(new)
}
