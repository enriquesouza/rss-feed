use crate::clustering::buckets::{infer_editorial_bucket, infer_topic_tags, topic_bucket_cap};
use crate::clustering::overlap::{has_specific_signature_overlap, same_story, signature_overlap};
use crate::clustering::scoring::compare_topic_cluster_priority;
use crate::clustering::signatures::topic_signature;
use crate::curation::priority::news_priority_score;
use crate::models::rss::channel_row::ChannelRow;
use crate::models::topic_cluster::TopicCluster;
use std::collections::BTreeMap;

const MAX_TOPIC_CLUSTERS_FOR_LLM: usize = 18;

pub fn cluster_news_for_llm(news: &[ChannelRow]) -> Vec<TopicCluster> {
    let mut candidates = news.to_vec();
    candidates.sort_by_cached_key(|item| std::cmp::Reverse(news_priority_score(item)));

    let mut clusters: Vec<TopicCluster> = Vec::new();

    for item in candidates {
        let bucket = infer_editorial_bucket(&item);
        let signature = topic_signature(&item);
        let tags = infer_topic_tags(&item, &signature, &bucket);

        if let Some(cluster) = clusters
            .iter_mut()
            .find(|cluster| should_merge_topic_cluster(cluster, &item, &bucket, &signature))
        {
            merge_topic_cluster(cluster, item, signature, tags);
        } else {
            clusters.push(TopicCluster {
                bucket,
                signature,
                tags,
                items: vec![item],
            });
        }
    }

    for cluster in &mut clusters {
        cluster
            .items
            .sort_by_cached_key(|item| std::cmp::Reverse(news_priority_score(item)));
    }

    clusters.sort_by(compare_topic_cluster_priority);

    let mut selected = Vec::new();
    let mut per_bucket: BTreeMap<String, usize> = BTreeMap::new();

    for cluster in clusters {
        let bucket_count = per_bucket.entry(cluster.bucket.clone()).or_default();
        if *bucket_count >= topic_bucket_cap(&cluster.bucket) {
            continue;
        }

        *bucket_count += 1;
        selected.push(cluster);

        if selected.len() >= MAX_TOPIC_CLUSTERS_FOR_LLM {
            break;
        }
    }

    selected
}

pub fn should_merge_topic_cluster(
    cluster: &TopicCluster,
    item: &ChannelRow,
    bucket: &str,
    signature: &[String],
) -> bool {
    if cluster.bucket != bucket {
        return false;
    }

    if cluster
        .items
        .iter()
        .any(|existing| same_story(existing, item))
    {
        return true;
    }

    let overlap = signature_overlap(&cluster.signature, signature);
    if overlap >= 2 {
        return true;
    }

    overlap >= 1 && has_specific_signature_overlap(&cluster.signature, signature)
}

pub fn merge_topic_cluster(
    cluster: &mut TopicCluster,
    item: ChannelRow,
    signature: Vec<String>,
    tags: Vec<String>,
) {
    cluster.items.push(item);

    cluster.signature.extend(signature);
    cluster.signature.sort();
    cluster.signature.dedup();
    cluster.signature.truncate(10);

    cluster.tags.extend(tags);
    cluster.tags.sort();
    cluster.tags.dedup();
}
