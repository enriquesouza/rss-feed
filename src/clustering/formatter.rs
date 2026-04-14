use crate::clustering::scoring::distinct_source_count;
use crate::models::topic_cluster::TopicCluster;
const MAX_ARTICLES_PER_TOPIC_CLUSTER: usize = 6;

pub fn format_topic_cluster_for_llm(cluster: &TopicCluster) -> String {
    let repeated_level = match cluster.items.len() {
        0 | 1 => "single_source",
        2 | 3 => "repeated",
        _ => "very_repeated",
    };

    let sample_titles = cluster
        .items
        .iter()
        .take(4)
        .map(|item| format!("- {}", item.title))
        .collect::<Vec<_>>()
        .join("\n");

    let articles = cluster
        .items
        .iter()
        .take(MAX_ARTICLES_PER_TOPIC_CLUSTER)
        .map(|item| {
            format!(
                "- [{}] {} | {}\n  {}",
                item.source, item.pub_date, item.title, item.sanitized_description
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "[TOPIC_BUCKET] {}\n[REPETITION_SIGNAL] {}\n[MERGED_ITEM_COUNT] {}\n[DISTINCT_SOURCE_COUNT] {}\n[TOPIC_SIGNATURE] {}\n[TAGS] {}\n[SAMPLE_TITLES]\n{}\n[ARTICLES]\n{}",
        cluster.bucket,
        repeated_level,
        cluster.items.len(),
        distinct_source_count(cluster),
        cluster.signature.join(", "),
        cluster.tags.join(", "),
        sample_titles,
        articles
    )
}
