use crate::formatters::text::parse_feed_datetime;
use crate::formatters::text::source_label;
use crate::models::configs::config::CURATION_CONFIG;
use crate::models::rss::channel_row::ChannelRow;
use crate::rss::lookback::lookback_days_for_feed;
use ::futures::future::join_all;
use chrono::Local;
use feed_rs::parser;
use std::error::Error;
use std::io::Cursor;

pub async fn get_rss_news(client: &reqwest::Client) -> Result<Vec<ChannelRow>, Box<dyn Error>> {
    let rss_providers = &CURATION_CONFIG.rss_providers;

    let fetched_news_results = join_all(rss_providers.iter().map(|rss_provider| async move {
        (
            rss_provider,
            fetch_news_from_web(client, rss_provider).await,
        )
    }))
    .await;

    let mut news: Vec<ChannelRow> = vec![];
    let mut success_count = 0usize;

    for (_, fetched_news) in fetched_news_results {
        if let Ok(fetched_news) = fetched_news {
            success_count += 1;
            news.extend(fetched_news);
        }
    }

    if success_count == 0 {
        return Err("all RSS providers failed".into());
    }

    news.sort_by_key(|f: &ChannelRow| parse_feed_datetime(&f.pub_date));
    news.reverse();

    Ok(news)
}

pub async fn fetch_news_from_web(
    client: &reqwest::Client,
    rss_provider: &str,
) -> Result<Vec<ChannelRow>, Box<dyn Error>> {
    let source = source_label(rss_provider);
    let lookback_days = lookback_days_for_feed(rss_provider);

    let req = client.get(rss_provider).send().await?.bytes().await?;

    let feed = parser::parse(Cursor::new(req.as_ref()))?;

    let today = Local::now().date_naive();
    let start_date = today - chrono::Duration::days(lookback_days - 1);

    let dates: Vec<ChannelRow> = feed
        .entries
        .into_iter()
        .filter_map(|entry| {
            let title = entry.title.map(|title| title.content).unwrap_or_default();
            let parsed = entry.published.or(entry.updated)?;
            let date_str = parsed.with_timezone(&Local).date_naive();

            if date_str < start_date || date_str > today {
                return None;
            }

            let description = entry
                .summary
                .map(|summary| summary.content)
                .or_else(|| entry.content.and_then(|content| content.body))
                .unwrap_or_default();

            Some(ChannelRow {
                source: source.clone(),
                title,
                link: entry
                    .links
                    .into_iter()
                    .find(|link| link.rel.as_deref() != Some("self"))
                    .map(|link| link.href)
                    .unwrap_or_default(),
                description,
                pub_date: parsed.to_rfc3339(),
            })
        })
        .collect();

    Ok(dates)
}
