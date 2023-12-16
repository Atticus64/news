use scraper::{Html, Selector};
use std::error::Error;
use terminal_spinners::{SpinnerBuilder, DOTS, DOTS2, MOON};

use crate::{
    lang::Lang,
    page::markdown::{get_markdown_content, show_news},
    scrape::ia::get_ia_new_resume,
    tui::select::get_answer,
};
use crate::utils::manage_error;

use super::issues::Issue;

#[derive(Debug)]
pub struct NewsLink {
    pub title: String,
    pub link: String,
}


pub fn get_lang_news_options(lang: &Lang, novelty: &Issue) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let (news, opts) = match lang {
        Lang::JavaScript => get_js_news(novelty.link.as_str())?,
        Lang::Rust => get_rs_news(novelty.link.as_str())?,
        Lang::Go => get_go_news(novelty.link.as_str())?,
        Lang::Python => get_py_news(novelty.link.as_str())?,
        Lang::Php => get_php_news(novelty.link.as_str())?,
        Lang::Cpp => get_cpp_news(novelty.link.as_str())?,
    };

    Ok((news, opts))
}

pub fn get_lang_news(lang: &Lang, novelty: &Issue) -> Result<Vec<NewsLink>, Box<dyn Error>> {
    let (news, _) = get_lang_news_options(&lang, &novelty)?;

    Ok(news)
}

pub fn get_news_by_lang_and_show(lang: &Lang, novelty: &Issue) -> Result<(), Box<dyn Error>> {
    let (news, options) = get_lang_news_options(&lang, &novelty)?;

    let answer = get_answer("What new do you like to watch?", options, "No new provided");

    let new_struct = news.iter().find(|new| new.title == answer);

    show_news(new_struct.expect("Failed to get new"))?;

    Ok(())
}

pub fn get_news_by_lang_and_summary(lang: &Lang, novelty: &Issue) -> Result<(), Box<dyn Error>> {
    let (news, options): (Vec<NewsLink>, Vec<String>) = get_lang_news_options(&lang, &novelty)?;

    let answer = get_answer("What new do you like to watch?", options, "No new provided");

    let new_struct = news.iter().find(|new| new.title == answer);

    let link = new_struct.expect("Failed to get new").link.to_string();
    get_ia_new_resume(link.to_string())?;

    println!("novelty link: {}", link);

    Ok(())
}

pub fn get_html(url: &str) -> String {
    let response = match ureq::get(url).call() {
        Ok(res) => res,
        Err(e) => {
            manage_error(e.to_string().as_str(), None);
            std::process::exit(1);
        }
    };

    response
        .into_string()
        .expect("Failed to convert to string html response")
}

fn fill_news(doc: Html, selector: Selector, tag: &str) -> Vec<NewsLink> {

    let mut links_vec = vec![];

    for elem in doc.select(&selector) {
        let ancor = Selector::parse(tag).expect("Failed to get");

        let ancors = elem.select(&ancor);

        for a in ancors {
            let text = get_markdown_content(a.inner_html().as_str());

            let node = a.value();

            let href = node.attr("href").expect("Failed to get href rs news");

            if !text.is_empty() {
                let new = NewsLink {
                    title: text.trim().to_string(),
                    link: href.to_string(),
                };
                links_vec.push(new)
            }
        }
    }

    links_vec
}

/// Search for javascript news of a specific issue
/// And return two arrays one of the news object and other of options of news to search
pub fn get_js_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&MOON)
        .text("Fetching JavaScript news")
        .start();

    let text = get_html(url);

    let document = Html::parse_document(&text);

    let selector = Selector::parse(".mainlink").expect("Failed to select posts");

    let vec_news = fill_news(document, selector, "a");

    let news_options = vec_news.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_news, news_options))
}

/// Search for rust news of a specific issue
/// And return two arrays one of the news object and other of options of news to search
pub fn get_rs_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text(" Fetching Rust news")
        .start();

    let text = get_html(url);

    let document = Html::parse_document(&text);

    let selector = Selector::parse(".post-content").expect("Failed to select posts");
    let mut vec_issues: Vec<NewsLink> = vec![];
    let mut links: Vec<String> = vec![];
    for elem in document.select(&selector) {
        let fragment = Selector::parse("li").expect("Failed to get");
        let ancor = Selector::parse("a").expect("Failed to get");

        // let a_select = elem.select(&fragment).next().expect("failed");
        let list = elem.select(&fragment);
        for li in list {
            let ancors = li.select(&ancor);
            for a in ancors {
                let text = get_markdown_content(a.inner_html().as_str());

                let node = a.value();

                let href = node.attr("href").expect("Failed to get href rs news");

                if text.len() < 15 {
                    continue;
                }
                if links.iter().any(|l| l.contains(href)) {
                    continue;
                }

                if !text.is_empty() {
                    let low_text = text.to_lowercase();
                    if low_text.contains("meetup") || href.contains("meetup") {
                        continue; 
                    }
                    let new = NewsLink {
                        title: text.trim().to_string(),
                        link: href.to_string(),
                    };
                    vec_issues.push(new)
                }
                links.push(href.to_string());
            }
        }
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_issues, issues_options))
}

pub fn get_go_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&DOTS2)
        .text(" Fetching Go news")
        .start();

    let text = get_html(url);

    let document = Html::parse_document(&text);

    let selector = Selector::parse(".mainlink").expect("Failed to select posts");

    let vec_news = fill_news(document, selector, "a");

    let issues_options = vec_news.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_news, issues_options))
}

pub fn get_py_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text(" Fetching Python news")
        .start();

    let text = get_html(url);

    let document = Html::parse_document(&text);
    let selector = Selector::parse("span").expect("Failed to get");
    let mut vec_issues: Vec<NewsLink> = vec![];
    let mut links: Vec<String> = vec![];
    for elem in document.select(&selector) {
        let fragment = Selector::parse("a").expect("Failed to get");

        // let a_select = elem.select(&fragment).next().expect("failed");
        let a_select = elem.select(&fragment);
        for a in a_select {
            let text = a.inner_html();
            let text = get_markdown_content(text.as_str());

            let node = a.value();
            let href = node.attr("href").expect("Error getting url");

            // let content = html.collect::<Vec<_>>();
            if text.len() < 15 {
                continue;
            }
            if links.iter().any(|l| l.contains(href)) {
                continue;
            }

            if !text.is_empty() {
                let new = NewsLink {
                    title: text.trim().to_string(),
                    link: href.to_string(),
                };
                vec_issues.push(new)
            }

            links.push(href.to_string());
        }
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_issues, issues_options))
}

pub fn get_php_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&MOON)
        .text("Fetching Php news")
        .start();

    let text = get_html(url);

    let document = Html::parse_document(&text);

    let selector = Selector::parse(".newsletter-stories").expect("Failed to select posts");

    let vec_news = fill_news( document, selector, ".title");

    let news_options = vec_news.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_news, news_options))
}

pub fn get_cpp_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&MOON)
        .text("Fetching Cpp news")
        .start();

    let text = get_html(url);

    let document = Html::parse_document(&text);

    let selector = Selector::parse(".newsletter-stories").expect("Failed to select posts");

    let vec_news = fill_news(document, selector, ".title");

    let news_options = vec_news.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_news, news_options))
}
