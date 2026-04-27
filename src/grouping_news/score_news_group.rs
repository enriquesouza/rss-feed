use crate::app_data::news_group::NewsGroup;
use crate::app_data::settings::news_rules::NEWS_RULES;
use crate::picking_news::score_news::score_news;
use std::collections::BTreeSet;

pub fn score_news_group(group: &NewsGroup) -> i32 {
    let best_item_score = group.items.iter().map(score_news).max().unwrap_or_default();
    let repeat_bonus = (group.items.len().saturating_sub(1).min(6) as i32) * 18;
    let source_bonus = (count_unique_sources(group).saturating_sub(1).min(6) as i32) * 10;
    let group_bonus = NEWS_RULES
        .topic_groups
        .iter()
        .find(|item| item.group_name == group.group_name)
        .map(|item| item.priority_bonus)
        .unwrap_or(0);

    best_item_score + repeat_bonus + source_bonus + group_bonus
}

pub fn count_unique_sources(group: &NewsGroup) -> usize {
    group
        .items
        .iter()
        .map(|item| item.source.as_str())
        .collect::<BTreeSet<_>>()
        .len()
}
