use serde::Deserialize;
use std::collections::{HashMap, HashSet};

use crate::app_data::settings::group_rule::GroupRule;

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

pub static NEWS_RULES: std::sync::LazyLock<super::news_rules::NewsRules> =
    std::sync::LazyLock::new(|| {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let path = std::path::Path::new(&manifest_dir).join("src/prompts/news_rules.yml");
        let content =
            std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read {:?}", path));
        serde_norway::from_str(&content).expect("Failed to parse news_rules.yml")
    });
