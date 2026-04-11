use ::futures::future::try_join_all;
use chrono::{self, DateTime, Local};
use dotenvy::dotenv;
use html2text::from_read;
use reqwest::{self};
use rss::Channel;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::{env, error::Error};
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
    //parse_mode: String,
}

#[derive(Deserialize)]
struct TelegramResponse {
    ok: bool,
    result: Option<IgnoredAny>,
    description: Option<String>,
    error_code: Option<i64>,
}
// To show the types I just need to control + option
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let client = reqwest::Client::new();

    loop {
        let news: Result<Vec<ChannelRow>, Box<dyn Error>> = get_rss_news(&client).await;
        match news {
            Ok(news) => {
                let news_sent_to_telegram: Result<Vec<TelegramResponse>, Box<dyn Error>> =
                    send_via_telegram(&client, news).await;
                match news_sent_to_telegram {
                    Ok(news_sent_to_telegram) if news_sent_to_telegram.is_empty() => {
                        println!("No fresh news found today.");
                    }
                    Ok(news_sent_to_telegram) => {
                        for response in news_sent_to_telegram
                            .iter()
                            .filter(|response: &&TelegramResponse| !response.ok)
                        {
                            println!(
                                "Telegram API error: code={:?}, description={:?}, result_present={}",
                                response.error_code,
                                response.description,
                                response.result.is_some()
                            );
                        }

                        println!(
                            "Did we send it all successfully? {:?}",
                            news_sent_to_telegram.iter().all(|response| response.ok)
                        );
                    }
                    Err(err) => {
                        eprintln!("Error sending to Telegram: {err}");
                    }
                }
            }
            Err(err) => {
                eprintln!("Error fetching RSS: {err}");
            }
        }

        sleep(Duration::from_hours(3)).await;
    }
}

async fn get_rss_news(client: &reqwest::Client) -> Result<Vec<ChannelRow>, Box<dyn Error>> {
    let rss_providers: [&str; 2] = [
        "https://bitcoinmagazine.com/feed",
        "https://cointelegraph.com/feed",
    ];

    let fetched_news = try_join_all(
        rss_providers
            .into_iter()
            .map(|rss_provider| fetch_news_from_web(client, rss_provider)),
    )
    .await?;

    let mut news: Vec<ChannelRow> = fetched_news.into_iter().flatten().collect();

    news.sort_by_key(|f| DateTime::parse_from_rfc2822(&f.pub_date).ok());

    Ok(news)
}

async fn fetch_news_from_web(
    client: &reqwest::Client,
    rss_provider: &str,
) -> Result<Vec<ChannelRow>, Box<dyn Error>> {
    // Get the XML RSS format from the feed on web
    let req = client.get(rss_provider).send().await?.bytes().await?;

    // Convert to RSS format Channel
    let channel: Channel = Channel::read_from(&req[..])?;

    // Today
    let today = Local::now().format("%Y-%m-%d").to_string();

    // Filter by today's news
    let dates: Vec<ChannelRow> = channel
        .items()
        .iter()
        .filter_map(|item| {
            // The filter map returns None or Some, per item
            let pub_date_str = item.pub_date.as_deref()?;
            let parsed = DateTime::parse_from_rfc2822(pub_date_str).ok()?;
            let date_str = parsed.with_timezone(&Local).format("%Y-%m-%d").to_string();
            if date_str != today {
                return None;
            }

            Some(ChannelRow {
                title: item.title.clone().unwrap_or_default(),
                link: item.link.clone().unwrap_or_default(),
                description: item.description.clone().unwrap_or_default(),
                pub_date: item.pub_date.clone().unwrap_or_default(),
            })
        })
        .collect();

    Ok(dates)
}

async fn send_via_telegram(
    client: &reqwest::Client,
    news: Vec<ChannelRow>,
) -> Result<Vec<TelegramResponse>, Box<dyn Error>> {
    let mut responses: Vec<TelegramResponse> = vec![];

    if !news.is_empty() {
        let n = news.iter().map(
            async move |item| -> Result<TelegramResponse, Box<dyn Error>> {
                let clean_html = ammonia::clean(&item.description);
                let parsed_html_to_text = from_read(clean_html.as_bytes(), 5000)?;
                let formatted_news = format!("{}", parsed_html_to_text);

                let telegram_message = TelegramMessage {
                    chat_id: CONFIG.telegram_chat_id.clone(),
                    text: formatted_news,
                    //parse_mode: "HTML".to_string(),
                };

                let post = client
                    .post(CONFIG.telegram_send_message_url.clone())
                    .json(&telegram_message)
                    .send()
                    .await?;

                let post_response: TelegramResponse = post.json().await?;

                Ok(post_response)
            },
        );

        let v: Vec<TelegramResponse> = try_join_all(n).await?;
        responses.extend(v);
    }

    Ok(responses)
}
