use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Deserialize)]
pub struct GroupRule {
    #[serde(rename = "id")]
    pub group_name: String,
    pub keywords: Vec<String>,
    #[serde(rename = "max_count")]
    pub max_items: usize,
    #[serde(rename = "priority_bonus")]
    pub priority_bonus: i32,
}

#[derive(Clone, Deserialize)]
pub struct NewsRules {
    #[serde(rename = "rss_providers")]
    pub rss_feeds: Vec<String>,
    pub slow_feeds: Vec<String>,
    pub word_replacements: HashMap<String, String>,
    pub word_aliases: HashMap<String, String>,
    pub ignored_words: HashSet<String>,
    pub common_words: HashSet<String>,
    #[serde(rename = "topic_groups")]
    pub topic_groups: Vec<GroupRule>,
    #[serde(rename = "priority_keywords_80")]
    pub priority_keywords_80: Vec<String>,
    #[serde(rename = "priority_keywords_60")]
    pub priority_keywords_60: Vec<String>,
    #[serde(rename = "technical_or_security_keywords")]
    pub technical_or_security_keywords: Vec<String>,
    #[serde(rename = "low_quality_keywords")]
    pub low_quality_keywords: Vec<String>,
    #[serde(rename = "high_volume_sources")]
    pub high_volume_sources: Vec<String>,
}
