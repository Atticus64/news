use news::scrape::link::get_js_news;

#[tokio::test]
async fn test_get_js_news() {
    let (news, options) = get_js_news("https://javascriptweekly.com/issues/628")
        .await
        .expect("Failed to get");
    assert!(news.len() > 0, "News vector must be with items");
    assert!(options.len() > 0, "Options vector must be with items");
}
