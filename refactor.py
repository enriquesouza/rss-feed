import os
import re

with open("src/main.rs", "r") as f:
    orig = f.read()

def extract_function(name):
    # Match fn name(...) -> ... { ... }
    # Assume properly balanced top-level braces by matching until `\n}\n\n` or similar.
    # It's safer to extract by regex that captures the body up to the matching brace brace level.
    # We can try a simple matching technique for top-level functions.
    pattern = re.compile(rf"^(?:async )?fn {name}\(.*?^}}$", re.MULTILINE | re.DOTALL)
    match = pattern.search(orig)
    if not match:
        return ""
    # add pub
    func = match.group(0)
    func = func.replace(f"fn {name}(", f"pub fn {name}(")
    return func

def create_file(path, content, uses=""):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write(uses + "\n" + content)

# 1. Models
# Move TopicCluster out since it will be shared
topic_cluster_struct = """
use crate::models::rss::channel_row::ChannelRow;

#[derive(Debug, Clone)]
pub struct TopicCluster {
    pub bucket: String,
    pub signature: Vec<String>,
    pub tags: Vec<String>,
    pub items: Vec<ChannelRow>,
}
"""
create_file("src/models/topic_cluster.rs", topic_cluster_struct)

# Add it to models/mod.rs
with open("src/models/mod.rs", "a") as f:
    f.write("pub mod topic_cluster;\n")


# 2. Formatters
create_file("src/formatters/mod.rs", "pub mod html;\npub mod text;\n")
create_file("src/formatters/html.rs", extract_function("sanitize_rss_text"), "use html2text::from_read;\n")
create_file("src/formatters/text.rs", 
    extract_function("normalize_title") + "\n\n" + 
    extract_function("parse_feed_datetime") + "\n\n" + 
    extract_function("source_label"), 
    "use chrono::{DateTime, FixedOffset};\n")


# 3. RSS
create_file("src/rss/mod.rs", "pub mod fetch;\npub mod lookback;\n")
create_file("src/rss/lookback.rs", extract_function("lookback_days_for_feed"), "use crate::models::configs::config::CURATION_CONFIG;\n")
fetch_uses = """use ::futures::future::join_all;
use chrono::Local;
use feed_rs::parser;
use std::error::Error;
use std::io::Cursor;
use crate::models::configs::config::CURATION_CONFIG;
use crate::models::rss::channel_row::ChannelRow;
use crate::rss::lookback::lookback_days_for_feed;
use crate::formatters::text::source_label;
"""
create_file("src/rss/fetch.rs", extract_function("get_rss_news") + "\n\n" + extract_function("fetch_news_from_web"), fetch_uses)


# 4. Telegram
create_file("src/telegram/mod.rs", "pub mod formatter;\npub mod sender;\n")
create_file("src/telegram/formatter.rs", extract_function("normalize_llm_output_for_telegram"), "")
sender_uses = """use std::error::Error;
use crate::models::configs::config::CONFIG;
use crate::models::telegram::telegram_message::TelegramMessage;
use crate::models::telegram::telegram_response::TelegramResponse;
use crate::telegram::formatter::normalize_llm_output_for_telegram;
"""
create_file("src/telegram/sender.rs", extract_function("send_via_telegram2"), sender_uses)


# 5. Curation
create_file("src/curation/mod.rs", "pub mod engine;\npub mod priority;\npub mod signals;\npub mod sources;\n")
sources_uses = "use crate::models::rss::channel_row::ChannelRow;\nuse crate::models::configs::config::CURATION_CONFIG;\nuse crate::curation::signals::is_technical_or_security;\n"
create_file("src/curation/sources.rs", extract_function("source_cap") + "\n\n" + extract_function("is_high_volume_general_source"), sources_uses)

signals_uses = "use crate::models::rss::channel_row::ChannelRow;\nuse crate::models::configs::config::CURATION_CONFIG;\nuse crate::formatters::html::sanitize_rss_text;\n"
create_file("src/curation/signals.rs", extract_function("is_technical_or_security") + "\n\n" + extract_function("is_low_signal_item"), signals_uses)

priority_uses = "use crate::models::rss::channel_row::ChannelRow;\nuse crate::models::configs::config::CURATION_CONFIG;\nuse crate::curation::signals::{is_technical_or_security, is_low_signal_item};\nuse crate::curation::sources::is_high_volume_general_source;\nuse crate::formatters::html::sanitize_rss_text;\nuse crate::formatters::text::parse_feed_datetime;\n"
create_file("src/curation/priority.rs", extract_function("news_priority_score") + "\n\n" + extract_function("compare_news_priority"), priority_uses)

engine_uses = """use crate::models::rss::channel_row::ChannelRow;
use crate::curation::priority::compare_news_priority;
use crate::curation::signals::{is_technical_or_security, is_low_signal_item};
use crate::curation::sources::source_cap;
use crate::formatters::text::normalize_title;
use std::collections::BTreeMap;
"""
# get constants directly copied to engine
curation_constants = "const TARGET_TECHNICAL_ITEMS: usize = 40;\nconst MAX_ITEMS_FOR_LLM: usize = 80;\n\n"
create_file("src/curation/engine.rs", curation_constants + extract_function("curate_news_for_llm") + "\n\n" + extract_function("collect_ranked_news"), engine_uses)


# 6. Clustering
create_file("src/clustering/mod.rs", "pub mod engine;\npub mod buckets;\npub mod signatures;\npub mod scoring;\npub mod formatter;\npub mod overlap;\n")

# formatting
formatter_uses = "use crate::models::topic_cluster::TopicCluster;\nuse crate::clustering::scoring::distinct_source_count;\nuse crate::formatters::html::sanitize_rss_text;\nconst MAX_ARTICLES_PER_TOPIC_CLUSTER: usize = 6;\n"
create_file("src/clustering/formatter.rs", extract_function("format_topic_cluster_for_llm"), formatter_uses)

# scoring
scoring_uses = "use crate::models::topic_cluster::TopicCluster;\nuse crate::models::configs::config::CURATION_CONFIG;\nuse crate::curation::priority::news_priority_score;\nuse crate::formatters::text::parse_feed_datetime;\nuse chrono::{DateTime, FixedOffset};\nuse std::collections::BTreeSet;\n"
create_file("src/clustering/scoring.rs", extract_function("compare_topic_cluster_priority") + "\n\n" + extract_function("topic_cluster_priority") + "\n\n" + extract_function("newest_cluster_datetime") + "\n\n" + extract_function("distinct_source_count"), scoring_uses)

# overlap
overlap_uses = "use crate::models::rss::channel_row::ChannelRow;\nuse crate::clustering::signatures::is_generic_cluster_token;\nuse crate::formatters::text::normalize_title;\n"
create_file("src/clustering/overlap.rs", extract_function("same_story") + "\n\n" + extract_function("signature_overlap") + "\n\n" + extract_function("has_specific_signature_overlap"), overlap_uses)

# signatures
signatures_uses = "use crate::models::rss::channel_row::ChannelRow;\nuse crate::models::configs::config::CURATION_CONFIG;\n"
create_file("src/clustering/signatures.rs", extract_function("topic_signature") + "\n\n" + extract_function("normalize_topic_text") + "\n\n" + extract_function("canonical_topic_token") + "\n\n" + extract_function("is_topic_stopword") + "\n\n" + extract_function("is_generic_cluster_token"), signatures_uses)

# buckets
buckets_uses = "use crate::models::rss::channel_row::ChannelRow;\nuse crate::models::configs::config::CURATION_CONFIG;\nuse crate::clustering::signatures::is_generic_cluster_token;\nuse crate::formatters::html::sanitize_rss_text;\n"
create_file("src/clustering/buckets.rs", extract_function("infer_editorial_bucket") + "\n\n" + extract_function("infer_topic_tags") + "\n\n" + extract_function("topic_bucket_cap"), buckets_uses)

# engine
clustering_engine_uses = """use std::collections::BTreeMap;
use crate::models::rss::channel_row::ChannelRow;
use crate::models::topic_cluster::TopicCluster;
use crate::curation::priority::compare_news_priority;
use crate::clustering::buckets::{infer_editorial_bucket, infer_topic_tags, topic_bucket_cap};
use crate::clustering::signatures::topic_signature;
use crate::clustering::scoring::compare_topic_cluster_priority;
use crate::clustering::overlap::{should_merge_topic_cluster, same_story, signature_overlap, has_specific_signature_overlap};

const MAX_TOPIC_CLUSTERS_FOR_LLM: usize = 18;
"""
create_file("src/clustering/engine.rs", clustering_engine_uses + extract_function("cluster_news_for_llm") + "\n\n" + extract_function("should_merge_topic_cluster") + "\n\n" + extract_function("merge_topic_cluster"), clustering_engine_uses)


print("Done creating module files.")
