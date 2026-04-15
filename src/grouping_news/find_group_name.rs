use crate::app_data::rss_news::news_item::NewsItem;
use crate::app_data::settings::app_env::NEWS_RULES;
use crate::grouping_news::find_topic_words::is_common_word;

pub fn find_group_name(item: &NewsItem) -> String {
    let search_text = format!(
        "{} {} {} {}",
        item.source, item.title, item.link, item.clean_description
    )
    .to_lowercase();

    for group in &NEWS_RULES.topic_groups {
        if group.keywords.iter().any(|word| search_text.contains(word)) {
            return group.group_name.clone();
        }
    }

    "general-market".to_string()
}

pub fn build_group_tags(item: &NewsItem, words: &[String], group_name: &str) -> Vec<String> {
    let mut tags = vec![group_name.to_string()];

    for word in words {
        if tags.len() >= 5 {
            break;
        }

        if !is_common_word(word) && !tags.iter().any(|existing| existing == word) {
            tags.push(word.clone());
        }
    }

    if tags.len() == 1 {
        tags.push(item.source.clone());
    }

    tags
}

pub fn max_groups_allowed(group_name: &str) -> usize {
    NEWS_RULES
        .topic_groups
        .iter()
        .find(|item| item.group_name == group_name)
        .map(|item| item.max_items)
        .unwrap_or(2)
}
