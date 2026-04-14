use crate::models::configs::config::CURATION_CONFIG;

pub fn lookback_days_for_feed(rss_provider: &str) -> i64 {
    if CURATION_CONFIG
        .slower_technical_feeds
        .iter()
        .any(|domain| rss_provider.contains(domain))
    {
        14
    } else {
        1
    }
}
