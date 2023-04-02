use std::error::Error;

use crabquery::Document;
use terminal_spinners::{SpinnerBuilder, CLOCK, MOON};

use super::Issue;

pub async fn get_php_issues_news() -> Result<(Vec<Issue>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&CLOCK)
        .text("Fetching PHP Issues")
        .start();
    const PHP_WEEKLY: &str = "https://php.libhunt.com/newsletter/archive";

    let response = reqwest::get(PHP_WEEKLY).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let mut issues = doc.select("tr");

    let mut vec_issues: Vec<Issue> = vec![];
    issues.remove(0);
    for issue in issues {
        let raw_title = issue
            .children()
            .get(1)
            .unwrap()
            .children()
            .first()
            .unwrap()
            .text()
            .expect("Failed to get");
        let issue_number = issue
            .children()
            .get(1)
            .unwrap()
            .children()
            .first()
            .unwrap()
            .attr("href")
            .expect("Failed to get");
        let vec: Vec<_> = raw_title.split('-').collect();
        let title_str = *vec.get(1).expect("Failed to get");
        let title = String::from(title_str);
        let date = issue
            .children()
            .first()
            .unwrap()
            .text()
            .expect("Failed to get");
        let name = format!("{title} - {date}");
        let url_completed = format!("https://php.libhunt.com{issue_number}");
        let new = Issue {
            title: name,
            link: url_completed,
        };
        vec_issues.push(new);
    }

    let issues_options: Vec<_> = vec_issues.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_issues, issues_options))
}

pub async fn get_latest_php_issue() -> Result<Issue, Box<dyn Error>> {
    const PHP_WEEKLY: &str = "https://php.libhunt.com/newsletter/archive";

    let handle = SpinnerBuilder::new()
        .spinner(&MOON)
        .text("Fetching Php Last Issue")
        .start();

    let response = reqwest::get(PHP_WEEKLY).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let mut issues = doc.select("tr");

    issues.remove(0);
    let first = issues.first().expect("Failed get first issue");
    let raw_title = first
        .children()
        .get(1)
        .unwrap()
        .children()
        .first()
        .unwrap()
        .text()
        .expect("Failed to get");
    let issue_number = first
        .children()
        .get(1)
        .unwrap()
        .children()
        .first()
        .unwrap()
        .attr("href")
        .expect("Failed to get");
    let vec: Vec<_> = raw_title.split('-').collect();
    let title_str = *vec.get(1).expect("Failed to get");
    let title = String::from(title_str);
    let date = first
        .children()
        .first()
        .unwrap()
        .text()
        .expect("Failed to get");
    let name = format!("{title} - {date}");
    let url_completed = format!("https://php.libhunt.com{issue_number}");
    let new = Issue {
        title: name,
        link: url_completed,
    };

    handle.done();

    Ok(new)
}
