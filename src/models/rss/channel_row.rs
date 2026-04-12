use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Tabled, Serialize, Deserialize)]
pub struct ChannelRow {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: String,
}
