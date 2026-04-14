use crate::curation::priority::news_priority_score;
use crate::models::configs::config::CURATION_CONFIG;
use crate::models::topic_cluster::TopicCluster;
use std::collections::BTreeSet;

pub fn topic_cluster_priority(cluster: &TopicCluster) -> i32 {
    let best_item_score = cluster
        .items
        .iter()
        .map(news_priority_score)
        .max()
        .unwrap_or_default();
    let repetition_bonus = (cluster.items.len().saturating_sub(1).min(6) as i32) * 18;
    let source_bonus = (distinct_source_count(cluster).saturating_sub(1).min(6) as i32) * 10;
    let bucket_bonus = CURATION_CONFIG
        .editorial_buckets
        .iter()
        .find(|b| b.id == cluster.bucket)
        .map(|b| b.priority_bonus)
        .unwrap_or(0);

    best_item_score + repetition_bonus + source_bonus + bucket_bonus
}

pub fn distinct_source_count(cluster: &TopicCluster) -> usize {
    cluster
        .items
        .iter()
        .map(|item| item.source.as_str())
        .collect::<BTreeSet<_>>()
        .len()
}
