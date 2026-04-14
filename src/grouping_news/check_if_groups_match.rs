use crate::app_data::rss_news::news_item::NewsItem;
use crate::formatting_text::clean_text::clean_title;
use crate::grouping_news::find_topic_words::is_common_word;
use std::collections::HashSet;

pub fn same_story(left: &NewsItem, right: &NewsItem) -> bool {
    clean_title(&left.title) == clean_title(&right.title)
        || (!left.link.is_empty() && left.link == right.link)
}

pub fn shared_word_count(left: &[String], right: &[String]) -> usize {
    let left_set: HashSet<_> = left.iter().collect();
    let right_set: HashSet<_> = right.iter().collect();
    left_set.intersection(&right_set).count()
}

pub fn has_shared_important_word(left: &[String], right: &[String]) -> bool {
    left.iter()
        .any(|word| right.iter().any(|other| other == word) && !is_common_word(word))
}
