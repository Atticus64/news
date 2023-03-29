use crabquery::Document;
use std::error::Error;
use terminal_spinners::{SpinnerBuilder, DOTS, DOTS2, MOON};

use crate::{lang::Lang, tui::select::get_answer_str, page::markdown::show_news};

use super::issues::Issue;

#[derive(Debug)]
pub struct NewsLink {
    pub title: String,
    pub link: String,
}


pub async fn get_news_by_lang(lang: &Lang, novelty: &Issue) ->  Result<(), Box<dyn Error>> {
  let (news, options) = match lang {
    Lang::JavaScript => get_js_news(novelty.link.as_str()).await?,
    Lang::Rust => get_rs_news(novelty.link.as_str()).await?,
    Lang::Go => get_go_news(novelty.link.as_str()).await?,
    Lang::Python => get_py_news(novelty.link.as_str()).await?,
    Lang::Php => get_php_news(novelty.link.as_str()).await?,
    Lang::Cpp => get_cpp_news(novelty.link.as_str()).await?
  };

  let answer = get_answer_str("What new do you like to watch?", options, "No new provided");

  let new_struct = news.iter().find(|new| new.title == answer);

  show_news(new_struct.unwrap()).await?;

  Ok(())
}

/// Search for javascript news of a specific issue
/// And return two arrays one of the news object and other of options of news to search
pub async fn get_js_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&MOON)
        .text("Fetching JavaScript news")
        .start();

    let response = reqwest::get(url).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let elements_li = doc.select(".mainlink");
    let mut vec_news: Vec<NewsLink> = vec![];
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
            .text()
            .expect("fail to conver a text");

        if title.len() < 6 {
            title = elem
                .children()
                .get(1)
                .expect("fail to get first element children title")
                .text()
                .expect("fail to convert a text title children");
        }

        let new = NewsLink { title, link: uri };
        vec_news.push(new)
    }

    let news_options = vec_news.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_news, news_options))
}

/// Search for rust news of a specific issue
/// And return two arrays one of the news object and other of options of news to search
pub async fn get_rs_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text(" Fetching Rust news")
        .start();
    let response = reqwest::get(url).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let post = doc.select(".post-content");
    let elem = post.first().unwrap();
    let elements_li = elem.select("li");
    let mut vec_issues: Vec<NewsLink> = vec![];
    for elem in elements_li {
        let uri = elem
            .children()
            .first()
            .expect("Failed to get first element")
            .attr("href");
        let title = elem
            .children()
            .first()
            .expect("Failed to get first element")
            .text()
            .unwrap();

        if let Some(link) = uri {
            if !title.is_empty() {
                let new = NewsLink { title, link };
                vec_issues.push(new)
            }
        }
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_issues, issues_options))
}

pub async fn get_go_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&DOTS2)
        .text(" Fetching Go news")
        .start();

    let response = reqwest::get(url).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let elements_li = doc.select(".mainlink");
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
            .text()
            .expect("fail to conver a text");

        if title.len() < 6 {
            title = elem
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

    handle.done();

    Ok((vec_issues, issues_options))
}

pub async fn get_py_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text(" Fetching Python news")
        .start();
    let response = reqwest::get(url).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let elements_li = doc.select("span");
    let mut vec_issues: Vec<NewsLink> = vec![];
    let mut links: Vec<String> = vec![];
    for elem in elements_li {
        let element = elem.children();
        let Some(first) = element.first() else { 
          continue;
        };
        let source = first.attr("href");
        if let Some(link) = source {
            if links.iter().any(|l| l.contains(&link)) {
                continue;
            }
            let title = elem
                .children()
                .first()
                .expect("Failed to get first child element")
                .text()
                .expect("Failed to tranform to String");

            if !title.is_empty() {
                let new = NewsLink {
                    title,
                    link: link.clone(),
                };
                vec_issues.push(new)
            }

            links.push(link);
        }
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_issues, issues_options))
}


pub async fn get_php_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
  let handle = SpinnerBuilder::new()
      .spinner(&MOON)
      .text("Fetching Php news")
      .start();

  let response = reqwest::get(url).await?;

  let text = response.text().await?;

  let doc = Document::from(text);

  let stories = doc.select(".newsletter-stories");
  let elements_a = stories.first().unwrap().select(".title");
  let mut vec_news: Vec<NewsLink> = vec![];
  for elem in elements_a {
      let uri = elem
          .attr("href")
          .expect("fail to get attr href");

      let title = elem
          .text()
          .expect("fail to conver a text");


      let new = NewsLink { title, link: uri };
      vec_news.push(new)
  }

  let news_options = vec_news.iter().map(|new| new.title.clone()).collect();

  handle.done();

  Ok((vec_news, news_options))
}


pub async fn get_cpp_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
  let handle = SpinnerBuilder::new()
      .spinner(&MOON)
      .text("Fetching Cpp news")
      .start();

  let response = reqwest::get(url).await?;

  let text = response.text().await?;

  let doc = Document::from(text);

  let stories = doc.select(".newsletter-stories");
  let elements_a = stories.first().unwrap().select(".title");
  let mut vec_news: Vec<NewsLink> = vec![];
  for elem in elements_a {
      let uri = elem
          .attr("href")
          .expect("fail to get attr href");

      let title = elem
          .text()
          .expect("fail to conver a text");


      let new = NewsLink { title, link: uri };
      vec_news.push(new)
  }

  let news_options = vec_news.iter().map(|new| new.title.clone()).collect();

  handle.done();

  Ok((vec_news, news_options))
}
