use std::error::Error;

use crabquery::Document;
use terminal_spinners::{SpinnerBuilder, MOON};

use super::Issue;

/// Search for javascript weekly issues news
/// Return a array of Issues and options of that issues to search them
pub async fn get_js_issues_news() -> Result<(Vec<Issue>, Vec<String>), Box<dyn Error>> {
    const JAVASCRIPT_WEEKLY_URL: &str = "https://javascriptweekly.com/issues";

    let response = reqwest::get(JAVASCRIPT_WEEKLY_URL).await?;

    let text = response.text().await?;

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
        let url_completed = format!("{JAVASCRIPT_WEEKLY_URL}/{number_issue}");
        let name = issue.text().expect("failed to tranform to text name issue");
        let new = Issue {
            title: name,
            link: url_completed,
        };
        vec_issues.push(new);
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    Ok((vec_issues, issues_options))
}

pub async fn get_latest_js_issue() -> Result<Issue, Box<dyn Error>> {
    const JAVASCRIPT_WEEKLY_URL: &str = "https://javascriptweekly.com/issues";

    let handle = SpinnerBuilder::new()
        .spinner(&MOON)
        .text("Fetching JavaScript Last Issue")
        .start();

    let response = reqwest::get(JAVASCRIPT_WEEKLY_URL).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let issue = doc.select(".issue");

    let first = issue.first().expect("Failed to get first issue");

    let url = first
        .children()
        .first()
        .expect("failed to get first element url")
        .attr("href")
        .expect("failed to get attr href element url");

    let number_issue = url.split('/').last().expect("failed to get last url");
    let url_completed = format!("{JAVASCRIPT_WEEKLY_URL}/{number_issue}");
    let name = first.text().expect("failed to tranform to text name issue");
    let new = Issue {
        title: name,
        link: url_completed,
    };

    handle.done();

    Ok(new)
}
