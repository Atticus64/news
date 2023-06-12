use std::error::Error;

use crabquery::Document;
use terminal_spinners::{SpinnerBuilder, EARTH, MOON};

use crate::scrape::link::get_html;

use super::Issue;

const GO_WEEKLY_URL: &str = "https://golangweekly.com/issues";

pub fn get_go_issues_news() -> Result<(Vec<Issue>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&EARTH)
        .text("Fetching Go Issues")
        .start();

    let text = get_html(GO_WEEKLY_URL);

    let doc = Document::from(text);

    let issues = doc.select(".issue");

    let mut vec_issues: Vec<Issue> = vec![];
    for issue in issues {
        let url = issue
            .children()
            .first()
            .expect("failed to get first element url")
            .attr("href")
            .expect("failed to get attr href element url");
        let number_issue = url.split('/').last().expect("failed to get last url");
        let url_completed = format!("{GO_WEEKLY_URL}/{number_issue}");
        let name = issue.text().expect("failed to tranform to text name issue");
        let new = Issue {
            title: name,
            link: url_completed,
        };
        vec_issues.push(new);
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_issues, issues_options))
}

pub fn get_latest_go_issue() -> Result<Issue, Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&MOON)
        .text("Fetching Go Last Issue")
        .start();

    let text = get_html(GO_WEEKLY_URL);

    let doc = Document::from(text);

    let issues = doc.select(".issue");
    let first = issues.first().expect("Fail to get first issue");

    let url = first
        .children()
        .first()
        .expect("failed to get first element url")
        .attr("href")
        .expect("failed to get attr href element url");
    let number_issue = url.split('/').last().expect("failed to get last url");
    let url_complete = format!("{GO_WEEKLY_URL}/{number_issue}");
    let name = first.text().expect("failed to tranform to text name issue");
    let new = Issue {
        title: name,
        link: url_complete,
    };

    handle.done();

    Ok(new)
}

pub fn get_last_go_issue() -> Result<Issue, Box<dyn Error>> {
    let text = get_html(GO_WEEKLY_URL);

    let doc = Document::from(text);

    let issues = doc.select(".issue");
    let first = issues.first().expect("Fail to get first issue");

    let url = first
        .children()
        .first()
        .expect("failed to get first element url")
        .attr("href")
        .expect("failed to get attr href element url");
    let number_issue = url.split('/').last().expect("failed to get last url");
    let url_complete = format!("{GO_WEEKLY_URL}/{number_issue}");
    let name = first.text().expect("failed to tranform to text name issue");
    let new = Issue {
        title: name,
        link: url_complete,
    };


    Ok(new)
}

