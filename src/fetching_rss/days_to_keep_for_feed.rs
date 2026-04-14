use crate::app_data::settings::app_env::NEWS_RULES;

pub fn days_to_keep_for_feed(feed_url: &str) -> i64 {
    if NEWS_RULES
        .slow_feeds
        .iter()
        .any(|domain| feed_url.contains(domain))
    {
        14
    } else {
        1
    }
}
