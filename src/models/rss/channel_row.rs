use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Tabled, Serialize, Deserialize, Debug, Clone)]
pub struct ChannelRow {
    pub source: String,
    pub title: String,
    pub link: String,
    pub description: String,
    #[tabled(skip)]
    pub sanitized_description: String,
    pub pub_date: String,
}
