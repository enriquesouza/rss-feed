use crate::app_data::rss_news::news_item::NewsItem;
use crate::app_data::settings::app_env::NEWS_RULES;
use crate::picking_news::check_news::is_tech_or_security;

pub fn max_items_per_source(item: &NewsItem) -> usize {
    if is_tech_or_security(item) {
        4
    } else if is_high_volume_source(&item.source) {
        3
    } else {
        2
    }
}

pub fn is_high_volume_source(source: &str) -> bool {
    NEWS_RULES
        .high_volume_sources
        .iter()
        .any(|candidate| source.contains(candidate))
}
