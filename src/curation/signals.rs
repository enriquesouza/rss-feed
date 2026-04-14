use crate::formatters::html::sanitize_rss_text;
use crate::models::configs::config::CURATION_CONFIG;
use crate::models::rss::channel_row::ChannelRow;

pub fn is_technical_or_security(item: &ChannelRow) -> bool {
    let haystack = format!(
        "{} {} {} {}",
        item.source,
        item.title.to_lowercase(),
        item.link.to_lowercase(),
        sanitize_rss_text(&item.description).to_lowercase()
    );

    CURATION_CONFIG
        .technical_or_security_keywords
        .iter()
        .any(|needle| haystack.contains(needle))
}

pub fn is_low_signal_item(item: &ChannelRow) -> bool {
    let title = item.title.to_lowercase();
    CURATION_CONFIG
        .low_signal_keywords
        .iter()
        .any(|needle| title.contains(needle))
}
