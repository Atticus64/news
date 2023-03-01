use crabquery::Document;
use std::error::Error;

#[derive(Debug)]
pub struct NewsLink {
    pub title: String,
    pub link: String,
}
/// Search for javascript news of a specific issue
/// And return two arrays one of the news object and other of options of news to search
pub async fn get_js_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let response = reqwest::get(url).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let elements_li = doc.select(".desc");
    let mut vec_issues: Vec<NewsLink> = vec![];
    for elem in elements_li {
        let uri = elem
            .select("a")
            .first()
            .expect("fail to get first element of uri")
            .attr("href")
            .expect("fail to get attr href");

        let mut title = elem
            .children()
            .first()
            .expect("fail to get first element")
            .children()
            .first()
            .expect("fail to get first element children 1")
            .text()
            .expect("fail to conver a text");

        if title.len() < 10 {
            title = elem
                .children()
                .first()
                .expect("fail to get first element title")
                .children()
                .get(1)
                .expect("fail to get first element children title")
                .text()
                .expect("fail to convert a text title children");
        }

        let new = NewsLink { title, link: uri };
        vec_issues.push(new)
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    Ok((vec_issues, issues_options))
}
