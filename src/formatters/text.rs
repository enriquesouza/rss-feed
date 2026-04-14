use chrono::DateTime;

pub fn normalize_title(title: &str) -> String {
    title
        .chars()
        .map(|character| {
            if character.is_alphanumeric() || character.is_whitespace() {
                character.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn parse_feed_datetime(value: &str) -> Option<DateTime<chrono::FixedOffset>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .or_else(|| DateTime::parse_from_rfc2822(value).ok())
}

pub fn source_label(rss_provider: &str) -> String {
    reqwest::Url::parse(rss_provider)
        .ok()
        .and_then(|url| url.host_str().map(str::to_string))
        .unwrap_or_else(|| rss_provider.to_string())
}
