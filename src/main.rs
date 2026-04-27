use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use dotenvy::dotenv;
use std::error::Error;
use std::time::Duration as StdDuration;
use tokio::time::{Duration, sleep};

pub mod app_data;
pub mod fetching_rss;
pub mod formatting_text;
pub mod grouping_news;
pub mod picking_news;
pub mod reading_news_today;
pub mod reading_stories_today;
pub mod sending_to_telegram;
pub mod writing_news;

use crate::app_data::rss_news::news_item::NewsItem;
use crate::app_data::telegram::telegram_response::TelegramResponse;
use crate::fetching_rss::fetch_rss_news::fetch_rss_news;
use crate::grouping_news::format_group_for_ai::format_group_for_ai;
use crate::grouping_news::group_related_news::{group_related_news, make_all_news_groups};
use crate::picking_news::pick_news_for_ai::pick_news_for_ai;
use crate::reading_news_today::save_and_check_news_read_today::NewsReadTodayDb;
use crate::reading_stories_today::save_and_check_stories_read_today::StoriesReadTodayDb;
use crate::sending_to_telegram::send_to_telegram::send_to_telegram;
use crate::writing_news::write_news_with_ai::NewsWriter;

use crate::grouping_news::score_news_group::count_unique_sources;

use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .connect_timeout(StdDuration::from_secs(10))
        .pool_idle_timeout(StdDuration::from_secs(90))
        .pool_max_idle_per_host(4)
        .redirect(reqwest::redirect::Policy::limited(5))
        .tcp_keepalive(StdDuration::from_secs(30))
        .user_agent("rss-feed/0.1")
        .build()?;
    let writer: NewsWriter = NewsWriter::new(&client);
    let news_read_today_db: NewsReadTodayDb = NewsReadTodayDb::open_news_read_today_db()?;
    let stories_read_today_db: StoriesReadTodayDb =
        StoriesReadTodayDb::open_stories_read_today_db()?;

    loop {
        let news_result: Result<Vec<NewsItem>, Box<dyn Error>> = fetch_rss_news(&client).await;

        match news_result {
            Ok(news_list) if news_list.is_empty() => {
                eprintln!("RSS returned no news items, sleeping 3h...");
            }
            Ok(news_list) => {
                let fresh_news =
                    news_read_today_db.filter_out_news_already_read_today(news_list)?;
                if fresh_news.is_empty() {
                    eprintln!("No fresh news after dedup, sleeping 3h...");
                    sleep(Duration::from_hours(3)).await;
                    continue;
                }

                news_read_today_db.save_news_read_today(&fresh_news)?;

                let story_groups = make_all_news_groups(&fresh_news);
                let fresh_story_groups = stories_read_today_db
                    .filter_out_stories_already_read_today(&client, story_groups)
                    .await?;
                if fresh_story_groups.is_empty() {
                    eprintln!("No fresh story groups after dedup, sleeping 3h...");
                    sleep(Duration::from_hours(3)).await;
                    continue;
                }

                stories_read_today_db.save_stories_read_today(&fresh_story_groups)?;

                let fresh_story_news = fresh_story_groups
                    .into_iter()
                    .flat_map(|group| group.items)
                    .collect::<Vec<_>>();

                let picked_news = pick_news_for_ai(&fresh_story_news);
                if picked_news.is_empty() {
                    eprintln!("No news picked for AI, sleeping 3h...");
                    sleep(Duration::from_hours(3)).await;
                    continue;
                }

                let news_groups: Vec<_> = group_related_news(&picked_news);

                if news_groups.is_empty() {
                    eprintln!("No news groups after grouping, sleeping 3h...");
                    sleep(Duration::from_hours(3)).await;
                    continue;
                }

                let processed_messages: Vec<String> = stream::iter(news_groups.into_iter())
                    .map(|group| {
                        let writer_ref = &writer;
                        async move {
                            let unique_sources = count_unique_sources(&group);
                            let prefix = match unique_sources {
                                1 => "[Falado em 1 blog]".to_string(),
                                n if n >= 4 => format!("[Muito falado +{} blogs]", n),
                                n => format!("[Falado em {} blogs]", n),
                            };
                            let group_text = format_group_for_ai(&group);
                            match writer_ref.write_news_message(group_text).await {
                                Ok(Some(msg)) => {
                                    let clean_msg =
                                        msg.replace("(Muito repetida)", "").trim().to_string();
                                    Some(format!("*{}*\n\n{}", prefix, clean_msg))
                                }
                                Ok(None) => {
                                    eprintln!("AI writer returned empty message for group: {}", prefix);
                                    None
                                }
                                Err(e) => {
                                    eprintln!("AI writer failed for group {}: {}", prefix, e);
                                    None
                                }
                            }
                        }
                    })
                    .buffer_unordered(5)
                    .filter_map(|res| async { res })
                    .collect()
                    .await;

                if processed_messages.is_empty() {
                    eprintln!("All AI writer calls failed or returned empty, sleeping 3h...");
                    sleep(Duration::from_hours(3)).await;
                    continue;
                }

                let final_message = processed_messages.join("\n\n");

                let mut parts = Vec::new();
                let mut start = 0;
                while start < final_message.len() {
                    let mut end = (start + 4096).min(final_message.len());
                    while end > start && !final_message.is_char_boundary(end) {
                        end -= 1;
                    }
                    parts.push(final_message[start..end].to_string());
                    start = end;
                }

                for part in parts {
                    let send_result: Result<TelegramResponse, Box<dyn Error>> =
                        send_to_telegram(&client, part).await;
                    if let Err(error) = send_result {
                        eprintln!("Could not send news to Telegram: {error}");
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("RSS fetch failed: {}", e);
            }
        }

        eprintln!("Sleeping 3 hours before next fetch...");
        sleep(Duration::from_hours(3)).await;
    }
}
