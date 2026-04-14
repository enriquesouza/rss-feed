use crate::app_data::rss_news::news_item::NewsItem;

#[derive(Debug, Clone)]
pub struct NewsGroup {
    pub group_name: String,
    pub topic_words: Vec<String>,
    pub tags: Vec<String>,
    pub items: Vec<NewsItem>,
}
