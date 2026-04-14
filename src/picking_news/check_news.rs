use crate::app_data::rss_news::news_item::NewsItem;
use crate::app_data::settings::app_env::NEWS_RULES;

pub fn is_tech_or_security(item: &NewsItem) -> bool {
    let search_text = format!(
        "{} {} {} {}",
        item.source,
        item.title.to_lowercase(),
        item.link.to_lowercase(),
        item.clean_description.to_lowercase()
    );

    NEWS_RULES
        .technical_or_security_keywords
        .iter()
        .any(|word| search_text.contains(word))
}

pub fn is_low_quality(item: &NewsItem) -> bool {
    let title = item.title.to_lowercase();
    NEWS_RULES
        .low_quality_keywords
        .iter()
        .any(|word| title.contains(word))
}
