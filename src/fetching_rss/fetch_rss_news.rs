use crate::app_data::rss_news::news_item::NewsItem;
use crate::app_data::settings::app_env::NEWS_RULES;
use crate::fetching_rss::days_to_keep_for_feed::days_to_keep_for_feed;
use crate::formatting_text::clean_html::clean_html_text;
use crate::formatting_text::clean_text::{get_source_name, parse_feed_date};
use ::futures::stream::{self, StreamExt};
use chrono::Local;
use feed_rs::parser;
use std::error::Error;
use std::io::Cursor;
use std::time::Duration;

const MAX_FEEDS_AT_ONCE: usize = 10;
const FEED_TIMEOUT_SECS: u64 = 20;

pub async fn fetch_rss_news(client: &reqwest::Client) -> Result<Vec<NewsItem>, Box<dyn Error>> {
    let feed_list = &NEWS_RULES.rss_feeds;

    let feed_jobs = stream::iter(feed_list.iter().cloned())
        .map(|feed_url| {
            let client = client.clone();

            async move { fetch_one_rss_feed(&client, &feed_url).await }
        })
        .buffer_unordered(MAX_FEEDS_AT_ONCE)
        .collect::<Vec<_>>()
        .await;

    let mut items: Vec<NewsItem> = vec![];
    let mut good_feeds = 0usize;

    for feed_items in feed_jobs {
        if let Ok(feed_items) = feed_items {
            good_feeds += 1;
            items.extend(feed_items);
        }
    }

    if good_feeds == 0 {
        return Err("all RSS providers failed".into());
    }

    items.sort_by_cached_key(|item| std::cmp::Reverse(parse_feed_date(&item.published_at)));

    Ok(items)
}

async fn fetch_one_rss_feed(
    client: &reqwest::Client,
    feed_url: &str,
) -> Result<Vec<NewsItem>, Box<dyn Error>> {
    let source = get_source_name(feed_url);
    let keep_days = days_to_keep_for_feed(feed_url);

    let feed_bytes = client
        .get(feed_url)
        .timeout(Duration::from_secs(FEED_TIMEOUT_SECS))
        .send()
        .await?
        .bytes()
        .await?;

    let feed = parser::parse(Cursor::new(feed_bytes.as_ref()))?;

    let today = Local::now().date_naive();
    let start_day = today - chrono::Duration::days(keep_days - 1);

    let items: Vec<NewsItem> = feed
        .entries
        .into_iter()
        .filter_map(|entry| {
            let title = entry.title.map(|title| title.content).unwrap_or_default();
            let date = entry.published.or(entry.updated)?;
            let local_date = date.with_timezone(&Local).date_naive();

            if local_date < start_day || local_date > today {
                return None;
            }

            let description = entry
                .summary
                .map(|summary| summary.content)
                .or_else(|| entry.content.and_then(|content| content.body))
                .unwrap_or_default();

            let clean_description = clean_html_text(&description);

            Some(NewsItem {
                source: source.clone(),
                title,
                link: entry
                    .links
                    .into_iter()
                    .find(|link| link.rel.as_deref() != Some("self"))
                    .map(|link| link.href)
                    .unwrap_or_default(),
                description,
                clean_description,
                published_at: date.to_rfc3339(),
            })
        })
        .collect();

    Ok(items)
}
