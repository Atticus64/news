use news_cli::scrape::issues::rust::get_rs_issues_news;

#[test]
fn test_get_js_news() {
    let (issues, options) = get_rs_issues_news().expect("Failed to get");

    assert!(issues.len() > 1, "News vector must be with items");
    assert!(options.len() > 1, "Options vector must be with items");
}
