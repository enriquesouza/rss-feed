use crate::app_data::rss_news::news_item::NewsItem;
use crate::app_data::settings::news_rules::NEWS_RULES;
use crate::grouping_news::count_group_points::{count_group_points, make_lower_news_text};
use crate::grouping_news::find_topic_words::is_common_word;

// A weak hint in the body alone is not enough to pick a topic.
const MIN_POINTS_TO_JOIN: i32 = 8;

// Picks the group with the most points, not the first group that matches.
pub fn find_group_name(item: &NewsItem) -> String {
    let text = make_lower_news_text(item);
    let mut best_name = "general-market".to_string();
    let mut best_points = 0;
    let mut best_bonus = 0;

    for group in &NEWS_RULES.topic_groups {
        let points = count_group_points(&text, group);

        if points < MIN_POINTS_TO_JOIN {
            continue;
        }

        let wins =
            points > best_points || (points == best_points && group.priority_bonus > best_bonus);

        if wins {
            best_points = points;
            best_bonus = group.priority_bonus;
            best_name = group.group_name.clone();
        }
    }

    best_name
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
