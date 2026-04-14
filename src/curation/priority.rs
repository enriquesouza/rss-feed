use crate::curation::sources::is_high_volume_general_source;
use crate::models::configs::config::CURATION_CONFIG;
use crate::models::rss::channel_row::ChannelRow;

pub fn news_priority_score(item: &ChannelRow) -> i32 {
    let haystack = format!(
        "{} {} {} {}",
        item.source,
        item.title.to_lowercase(),
        item.link.to_lowercase(),
        item.sanitized_description.to_lowercase()
    );

    let mut score = 0i32;

    let is_tech = CURATION_CONFIG
        .technical_or_security_keywords
        .iter()
        .any(|needle| haystack.contains(needle));

    if is_tech {
        score += 150;
    }

    if CURATION_CONFIG
        .priority_keywords_80
        .iter()
        .any(|needle| haystack.contains(needle))
    {
        score += 80;
    }

    if CURATION_CONFIG
        .priority_keywords_60
        .iter()
        .any(|needle| haystack.contains(needle))
    {
        score += 60;
    }

    if is_high_volume_general_source(&item.source) {
        score -= 20;
    }

    let title_lower = item.title.to_lowercase();
    let is_low = CURATION_CONFIG
        .low_signal_keywords
        .iter()
        .any(|needle| title_lower.contains(needle));

    if is_low {
        score -= 120;
    }

    score
}
