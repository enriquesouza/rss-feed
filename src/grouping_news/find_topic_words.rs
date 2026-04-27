use crate::app_data::rss_news::news_item::NewsItem;
use crate::app_data::settings::news_rules::NEWS_RULES;

pub fn find_topic_words(item: &NewsItem) -> Vec<String> {
    let clean_text = clean_words_text(&item.title);
    let mut words = Vec::new();

    for word in clean_text.split_whitespace() {
        if let Some(word) = base_word(word) {
            if is_ignored_word(word) {
                continue;
            }

            if !words.iter().any(|existing| existing == word) {
                words.push(word.to_string());
            }
        }
    }

    if words.is_empty() {
        words.push(item.source.to_lowercase());
    }

    words.truncate(8);
    words
}

fn clean_words_text(input: &str) -> String {
    let mut cleaned = input.to_lowercase();

    for (needle, replacement) in &NEWS_RULES.word_replacements {
        cleaned = cleaned.replace(needle, replacement);
    }

    cleaned
        .chars()
        .map(|character| {
            if character.is_alphanumeric() || character == '_' || character.is_whitespace() {
                character
            } else {
                ' '
            }
        })
        .collect()
}

fn base_word(word: &str) -> Option<&str> {
    let standard = NEWS_RULES
        .word_aliases
        .get(word)
        .map(String::as_str)
        .unwrap_or(word);

    if standard.len() <= 2 || standard.chars().all(|character| character.is_numeric()) {
        return None;
    }

    Some(standard)
}

fn is_ignored_word(token: &str) -> bool {
    NEWS_RULES.ignored_words.contains(token)
}

pub fn is_common_word(word: &str) -> bool {
    NEWS_RULES.common_words.contains(word)
}
