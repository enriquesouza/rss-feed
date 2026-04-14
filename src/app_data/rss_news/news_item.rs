use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Tabled, Serialize, Deserialize, Debug, Clone)]
pub struct NewsItem {
    pub source: String,
    pub title: String,
    pub link: String,
    pub description: String,
    #[tabled(skip)]
    pub clean_description: String,
    pub published_at: String,
}
