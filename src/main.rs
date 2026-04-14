use dotenvy::dotenv;
use std::error::Error;
use tokio::time::{Duration, sleep};

pub mod app_data;
pub mod fetching_rss;
pub mod formatting_text;
pub mod grouping_news;
pub mod picking_news;
pub mod sending_to_telegram;
pub mod writing_news;

use crate::app_data::rss_news::news_item::NewsItem;
use crate::app_data::telegram::telegram_response::TelegramResponse;
use crate::fetching_rss::fetch_rss_news::fetch_rss_news;
use crate::grouping_news::format_group_for_ai::format_group_for_ai;
use crate::grouping_news::group_related_news::group_related_news;
use crate::picking_news::pick_news_for_ai::pick_news_for_ai;
use crate::sending_to_telegram::send_to_telegram::send_to_telegram;
use crate::writing_news::write_news_with_ai::NewsWriter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let client: reqwest::Client = reqwest::Client::new();
    let writer: NewsWriter = NewsWriter::new(&client);

    loop {
        let news_result: Result<Vec<NewsItem>, Box<dyn Error>> = fetch_rss_news(&client).await;

        match news_result {
            Ok(news_list) if news_list.is_empty() => {}
            Ok(news_list) => {
                let picked_news = pick_news_for_ai(&news_list);

                let news_groups = group_related_news(&picked_news);

                let group_texts = news_groups
                    .iter()
                    .map(format_group_for_ai)
                    .collect::<Vec<_>>();

                let message = writer
                    .write_news_message(group_texts)
                    .await?
                    .unwrap_or_default();

                let letters: Vec<char> = message.chars().collect();
                let parts: Vec<String> = letters
                    .chunks(4096)
                    .map(|part| part.iter().collect())
                    .collect();

                for part in parts {
                    let send_result: Result<TelegramResponse, Box<dyn Error>> =
                        send_to_telegram(&client, part).await;

                    let _ = send_result;
                }
            }
            Err(_) => {}
        }

        sleep(Duration::from_hours(3)).await;
    }
}
