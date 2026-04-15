use crate::app_data::rss_news::news_item::NewsItem;
use crate::app_data::settings::app_env::NEWS_RULES;
use crate::picking_news::check_source_type::is_high_volume_source;

pub fn score_news(item: &NewsItem) -> i32 {
    let search_text = format!(
        "{} {} {} {}",
        item.source, item.title, item.link, item.clean_description
    )
    .to_lowercase();

    let mut score = 0i32;

    let is_tech = NEWS_RULES
        .technical_or_security_keywords
        .iter()
        .any(|word| search_text.contains(word));

    if is_tech {
        score += 150;
    }

    if NEWS_RULES
        .priority_keywords_80
        .iter()
        .any(|word| search_text.contains(word))
    {
        score += 80;
    }

    if NEWS_RULES
        .priority_keywords_60
        .iter()
        .any(|word| search_text.contains(word))
    {
        score += 60;
    }

    if is_high_volume_source(&item.source) {
        score -= 20;
    }

    let title_lower = item.title.to_lowercase();
    let is_low_quality = NEWS_RULES
        .low_quality_keywords
        .iter()
        .any(|word| title_lower.contains(word));

    if is_low_quality {
        score -= 120;
    }

    score
}
