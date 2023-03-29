use news::scrape::issues::rust::get_rs_issues_news;

#[tokio::test]
async fn test_get_js_news() {
    let (issues, options) = get_rs_issues_news().await.unwrap();

    assert!(issues.len() > 1, "News vector must be with items");
    assert!(options.len() > 1, "Options vector must be with items");
}
