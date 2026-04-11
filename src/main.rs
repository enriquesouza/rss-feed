use ::futures::future::try_join_all;
use chrono::{self, DateTime, Local};
use dotenvy::dotenv;
use html2text::from_read;
use reqwest::{self};
use rss::Channel;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::{env, error::Error, ops::Deref};
use tabled::Tabled;
use tokio::time::{Duration, sleep};

// The closure is NOT run here; it's saved for later.
static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|_| "missing TELEGRAM_BOT_TOKEN env var")
        .expect("TELEGRAM_BOT_TOKEN is needed");

    let telegram_chat_id = env::var("TELEGRAM_CHAT_ID")
        .map_err(|_| "missing TELEGRAM_CHAT_ID env var")
        .expect("TELEGRAM_CHAT_ID is needed");

    let telegram_send_message_url =
        format!("https://api.telegram.org/bot{telegram_bot_token}/sendMessage");

    let config = Config {
        telegram_chat_id: telegram_chat_id,
        telegram_send_message_url: telegram_send_message_url,
    };
    config
});

#[derive(Clone)]
struct Config {
    pub telegram_chat_id: String,
    pub telegram_send_message_url: String,
}

#[derive(Tabled, Serialize, Deserialize)]
struct ChannelRow {
    title: String,
    link: String,
    description: String,
    pub_date: String,
}

#[derive(Serialize)]
struct TelegramMessage {
    chat_id: String,
    text: String,
    parse_mode: String,
}
// To show the types I just need to control + option
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let client = reqwest::Client::new();

    loop {
        let news: Vec<ChannelRow> = get_rss_news().await?;
        let news_sent_to_telegram: Vec<String> = send_via_telegram(&client, news).await?;

        println!(
            "Did we send it all successfully? {:?}",
            news_sent_to_telegram.iter().all(|item| item.contains("ok"))
        );

        sleep(Duration::from_hours(3)).await;
    }
}

async fn get_rss_news() -> Result<Vec<ChannelRow>, Box<dyn Error>> {
    let rss_providers: [&str; 2] = [
        "https://bitcoinmagazine.com/feed",
        "https://cointelegraph.com/feed",
    ];

    let fetched_news = try_join_all(rss_providers.into_iter().map(fetch_news_from_web)).await?;
    let mut news: Vec<ChannelRow> = fetched_news.into_iter().flatten().collect();

    news.sort_by_key(|f: &ChannelRow| f.pub_date.clone());

    Ok(news)
}

async fn fetch_news_from_web(rss_provider: &str) -> Result<Vec<ChannelRow>, Box<dyn Error>> {
    let req = reqwest::get(rss_provider).await?.bytes().await?;

    let channel: Channel = Channel::read_from(&req[..])?;

    let dates: Vec<ChannelRow> = channel
        .items()
        .into_iter()
        .filter(|f| {
            f.pub_date.clone().is_some_and(|date| {
                let pub_date: &str = date.deref();

                let parsed_date: String = DateTime::parse_from_rfc2822(pub_date)
                    .ok()
                    .map(|dt| dt.with_timezone(&Local).format("%Y-%m-%d").to_string())
                    .unwrap();

                let now: String = Local::now().format("%Y-%m-%d").to_string();

                return parsed_date == now;
            })
        })
        .map(|item| ChannelRow {
            title: item.title.clone().unwrap(),
            link: item.link.clone().unwrap(),
            description: item.description.clone().unwrap(),
            pub_date: item.pub_date.clone().unwrap(),
        })
        .collect();
    Ok(dates)
}

async fn send_via_telegram(
    client: &reqwest::Client,
    news: Vec<ChannelRow>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut responses: Vec<String> = vec![];

    if news.is_empty() {
        responses.push("No fresh news found today.".to_string());
    } else {
        let n = news
            .iter()
            .map(async move |item| -> Result<String, Box<dyn Error>> {
                let clean_html = ammonia::clean(&item.description);
                let parsed_html_to_text = from_read(clean_html.as_bytes(), 100).unwrap();
                let formatted_news = format!("{}", parsed_html_to_text);

                let telegram_message = TelegramMessage {
                    chat_id: CONFIG.telegram_chat_id.clone(),
                    text: formatted_news,
                    parse_mode: "HTML".to_string(),
                };

                let post = client
                    .post(CONFIG.telegram_send_message_url.clone())
                    .json(&telegram_message)
                    .send()
                    .await?;

                let post_response_text = post.text().await?;

                Ok(post_response_text)
            });

        let v: Vec<String> = try_join_all(n).await?;
        responses.extend(v);
    };

    Ok(responses)
}
