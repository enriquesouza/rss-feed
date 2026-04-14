use dotenvy::dotenv;
use std::error::Error;
use tokio::time::{Duration, sleep};

pub mod clustering;
pub mod curation;
pub mod formatters;
pub mod models;
pub mod rss;
pub mod services;
pub mod telegram;

use crate::clustering::engine::cluster_news_for_llm;
use crate::clustering::formatter::format_topic_cluster_for_llm;
use crate::curation::engine::curate_news_for_llm;
use crate::models::rss::channel_row::ChannelRow;
use crate::models::telegram::telegram_response::TelegramResponse;
use crate::rss::fetch::get_rss_news;
use crate::services::open_router_service::OpenRouterService;
use crate::telegram::sender::send_via_telegram2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let client: reqwest::Client = reqwest::Client::new();
    let open_router_service: OpenRouterService = OpenRouterService::new(&client);

    loop {
        let news: Result<Vec<ChannelRow>, Box<dyn Error>> = get_rss_news(&client).await;

        match news {
            Ok(news) if news.is_empty() => {}
            Ok(news) => {
                let curated_news = curate_news_for_llm(&news);

                let topic_clusters = cluster_news_for_llm(&curated_news);

                let news_to_string = topic_clusters
                    .iter()
                    .map(format_topic_cluster_for_llm)
                    .collect::<Vec<_>>();

                let optimized_news = open_router_service
                    .get_optimized_news(news_to_string)
                    .await?
                    .unwrap_or_default();

                let char_vec: Vec<char> = optimized_news.chars().collect();
                let chunks: Vec<String> = char_vec
                    .chunks(4096)
                    .map(|chunk| chunk.iter().collect())
                    .collect();

                for chunk in chunks.into_iter() {
                    let news_sent_to_telegram: Result<TelegramResponse, Box<dyn Error>> =
                        send_via_telegram2(&client, chunk).await;

                    let _ = news_sent_to_telegram;
                }
            }
            Err(_) => {}
        }

        sleep(Duration::from_hours(3)).await;
    }
}
