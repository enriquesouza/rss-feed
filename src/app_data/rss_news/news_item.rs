use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewsItem {
    pub source: String,
    pub title: String,
    pub link: String,
    pub description: String,
    pub clean_description: String,
    pub published_at: String,
}
