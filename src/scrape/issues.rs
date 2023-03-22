use crabquery::Document;
use std::error::Error;
use terminal_spinners::{SpinnerBuilder, CLOCK, EARTH, FLIP};

#[derive(Debug)]
pub struct Issue {
    pub title: String,
    pub link: String,
}

/// Search for javascript weekly issues news
/// Return a array of Issues and options of that issues to search them
pub async fn get_js_issues_news() -> Result<(Vec<Issue>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&CLOCK)
        .text("Fetching JavaScript Issues")
        .start();
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

    handle.done();

    Ok((vec_issues, issues_options))
}

pub async fn get_rs_issues_news() -> Result<(Vec<Issue>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&FLIP)
        .text(" Fetching Rust Issues")
        .start();
    const RUST_WEEKLY_URL: &str = "https://this-week-in-rust.org/blog/archives/index.html";

    let response = reqwest::get(RUST_WEEKLY_URL).await?;

    let text = response.text().await?;

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

pub async fn get_go_issues_news() -> Result<(Vec<Issue>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&EARTH)
        .text("Fetching Go Issues")
        .start();
    const GO_WEEKLY_URL: &str = "https://golangweekly.com/issues";

    let response = reqwest::get(GO_WEEKLY_URL).await?;

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

pub async fn get_py_issues_news() -> Result<(Vec<Issue>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&FLIP)
        .text(" Fetching Python Issues")
        .start();
    const PYTHON_WEEKLY_URL: &str = "https://pycoders.com/issues";

    let response = reqwest::get(PYTHON_WEEKLY_URL).await?;

    let text = response.text().await?;

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
            .split("/")
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
