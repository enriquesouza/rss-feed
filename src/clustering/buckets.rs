use crate::clustering::signatures::is_generic_cluster_token;
use crate::formatters::html::sanitize_rss_text;
use crate::models::configs::config::CURATION_CONFIG;
use crate::models::rss::channel_row::ChannelRow;

pub fn infer_editorial_bucket(item: &ChannelRow) -> String {
    let haystack = format!(
        "{} {} {} {}",
        item.source,
        item.title.to_lowercase(),
        item.link.to_lowercase(),
        sanitize_rss_text(&item.description).to_lowercase()
    );

    for bucket in &CURATION_CONFIG.editorial_buckets {
        if bucket.keywords.iter().any(|kw| haystack.contains(kw)) {
            return bucket.id.clone();
        }
    }

    "general-market".to_string()
}

pub fn infer_topic_tags(item: &ChannelRow, signature: &[String], bucket: &str) -> Vec<String> {
    let mut tags = vec![bucket.to_string()];

    for token in signature {
        if tags.len() >= 5 {
            break;
        }

        if !is_generic_cluster_token(token) && !tags.iter().any(|existing| existing == token) {
            tags.push(token.clone());
        }
    }

    if tags.len() == 1 {
        tags.push(item.source.clone());
    }

    tags
}

pub fn topic_bucket_cap(bucket: &str) -> usize {
    CURATION_CONFIG
        .editorial_buckets
        .iter()
        .find(|b| b.id == bucket)
        .map(|b| b.cap)
        .unwrap_or(2)
}
