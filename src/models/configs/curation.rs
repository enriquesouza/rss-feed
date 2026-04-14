use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Deserialize)]
pub struct EditorialBucket {
    pub id: String,
    pub keywords: Vec<String>,
    pub cap: usize,
    pub priority_bonus: i32,
}

#[derive(Clone, Deserialize)]
pub struct CurationConfig {
    pub rss_providers: Vec<String>,
    pub slower_technical_feeds: Vec<String>,
    pub topic_normalization: HashMap<String, String>,
    pub canonical_tokens: HashMap<String, String>,
    pub topic_stopwords: HashSet<String>,
    pub generic_cluster_tokens: HashSet<String>,
    pub editorial_buckets: Vec<EditorialBucket>,
    pub priority_keywords_80: Vec<String>,
    pub priority_keywords_60: Vec<String>,
    pub technical_or_security_keywords: Vec<String>,
    pub low_signal_keywords: Vec<String>,
    pub high_volume_general_sources: Vec<String>,
}
