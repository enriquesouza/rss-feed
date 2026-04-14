use crate::models::configs::config::CURATION_CONFIG;
use crate::models::rss::channel_row::ChannelRow;

pub fn topic_signature(item: &ChannelRow) -> Vec<String> {
    let normalized = normalize_topic_text(&item.title);
    let mut tokens = Vec::new();

    for token in normalized.split_whitespace() {
        if let Some(token) = canonical_topic_token(token) {
            if is_topic_stopword(&token) {
                continue;
            }

            if !tokens.iter().any(|existing| existing == &token) {
                tokens.push(token);
            }
        }
    }

    if tokens.is_empty() {
        tokens.push(item.source.to_lowercase());
    }

    tokens.truncate(8);
    tokens
}

pub fn normalize_topic_text(input: &str) -> String {
    let mut normalized = input.to_lowercase();

    for (needle, replacement) in &CURATION_CONFIG.topic_normalization {
        normalized = normalized.replace(needle, replacement);
    }

    normalized
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

pub fn canonical_topic_token(token: &str) -> Option<String> {
    let canonical = CURATION_CONFIG
        .canonical_tokens
        .get(token)
        .map(String::as_str)
        .unwrap_or(token);

    if canonical.len() <= 2 || canonical.chars().all(|character| character.is_numeric()) {
        return None;
    }

    Some(canonical.to_string())
}

pub fn is_topic_stopword(token: &str) -> bool {
    CURATION_CONFIG.topic_stopwords.contains(token)
}

pub fn is_generic_cluster_token(token: &str) -> bool {
    CURATION_CONFIG.generic_cluster_tokens.contains(token)
}
