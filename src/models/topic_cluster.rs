use crate::models::rss::channel_row::ChannelRow;

#[derive(Debug, Clone)]
pub struct TopicCluster {
    pub bucket: String,
    pub signature: Vec<String>,
    pub tags: Vec<String>,
    pub items: Vec<ChannelRow>,
}
