use crate::app_data::news_group::NewsGroup;
use crate::app_data::rss_news::news_item::NewsItem;
use crate::grouping_news::check_if_groups_match::{
    has_shared_important_word, same_story, shared_word_count,
};
use crate::grouping_news::find_group_name::{
    build_group_tags, find_group_name, max_groups_allowed,
};
use crate::grouping_news::find_topic_words::find_topic_words;
use crate::grouping_news::score_news_group::score_news_group;
use crate::picking_news::score_news::score_news;
use std::collections::BTreeMap;

const MAX_GROUPS: usize = 18;

pub fn group_related_news(news: &[NewsItem]) -> Vec<NewsGroup> {
    let mut sorted_items = news.to_vec();
    sorted_items.sort_by_cached_key(|item| std::cmp::Reverse(score_news(item)));

    let mut news_groups: Vec<NewsGroup> = Vec::new();

    for item in sorted_items {
        let group_name = find_group_name(&item);
        let topic_words = find_topic_words(&item);
        let tags = build_group_tags(&item, &topic_words, &group_name);

        if let Some(group) = news_groups
            .iter_mut()
            .find(|group| can_join_group(group, &item, &group_name, &topic_words))
        {
            add_to_group(group, item, topic_words, tags);
        } else {
            news_groups.push(NewsGroup {
                group_name,
                topic_words,
                tags,
                items: vec![item],
            });
        }
    }

    for group in &mut news_groups {
        group
            .items
            .sort_by_cached_key(|item| std::cmp::Reverse(score_news(item)));
    }

    news_groups.sort_by_cached_key(|group| std::cmp::Reverse(score_news_group(group)));

    let mut selected = Vec::new();
    let mut group_counts: BTreeMap<String, usize> = BTreeMap::new();

    for group in news_groups {
        let group_total = group_counts.entry(group.group_name.clone()).or_default();
        if *group_total >= max_groups_allowed(&group.group_name) {
            continue;
        }

        *group_total += 1;
        selected.push(group);

        if selected.len() >= MAX_GROUPS {
            break;
        }
    }

    selected
}

fn can_join_group(
    group: &NewsGroup,
    item: &NewsItem,
    group_name: &str,
    topic_words: &[String],
) -> bool {
    if group.group_name != group_name {
        return false;
    }

    if group
        .items
        .iter()
        .any(|existing| same_story(existing, item))
    {
        return true;
    }

    let shared_words = shared_word_count(&group.topic_words, topic_words);
    if shared_words >= 2 {
        return true;
    }

    shared_words >= 1 && has_shared_important_word(&group.topic_words, topic_words)
}

fn add_to_group(
    group: &mut NewsGroup,
    item: NewsItem,
    topic_words: Vec<String>,
    tags: Vec<String>,
) {
    group.items.push(item);

    group.topic_words.extend(topic_words);
    group.topic_words.sort();
    group.topic_words.dedup();
    group.topic_words.truncate(10);

    group.tags.extend(tags);
    group.tags.sort();
    group.tags.dedup();
}
