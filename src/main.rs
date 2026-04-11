use ::futures::future::try_join_all;
use chrono::{self, DateTime, Local};
use dotenvy::dotenv;
use html2text::from_read;
use reqwest::{self};
use rss::Channel;
use serde::{Deserialize, Serialize};
use std::{env, error::Error, ops::Deref, thread};
use tabled::Tabled;
use tokio::time::{Duration, sleep};

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

    loop {
        get_rss_news().await?;
        sleep(Duration::from_hours(3)).await;
    }
}

async fn get_rss_news() -> Result<(), Box<dyn Error>> {
    let rss_providers: [&str; 2] = [
        "https://bitcoinmagazine.com/feed",
        "https://cointelegraph.com/feed",
    ];

    let mut news: Vec<ChannelRow> = vec![];

    for rss_provider in rss_providers {
        let threads = thread::spawn(async move || -> Result<Vec<ChannelRow>, Box<dyn Error>> {
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
        });
        let future = threads.join().unwrap();
        let res: Vec<ChannelRow> = future.await?;
        news.extend(res);
    }

    news.sort_by_key(|f: &ChannelRow| f.pub_date.clone());

    send_via_telegram(news).await?;

    Ok(())
}

async fn send_via_telegram(mut news: Vec<ChannelRow>) -> Result<(), Box<dyn Error>> {
    if news.is_empty() {
        "No fresh news found today.".to_string()
    } else {
        let n = news
            .iter()
            .map(async move |item| -> Result<String, Box<dyn Error>> {
                let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN")
                    .map_err(|_| "missing TELEGRAM_BOT_TOKEN env var")
                    .expect("TELEGRAM_BOT_TOKEN is needed");

                let telegram_chat_id = env::var("TELEGRAM_CHAT_ID")
                    .map_err(|_| "missing TELEGRAM_CHAT_ID env var")
                    .expect("TELEGRAM_CHAT_ID is needed");

                let telegram_send_message_url =
                    format!("https://api.telegram.org/bot{telegram_bot_token}/sendMessage");

                let clean_html = ammonia::clean(&item.description);
                let parsed_html_to_text = from_read(clean_html.as_bytes(), 100).unwrap();
                let formatted_news = format!("{}", parsed_html_to_text);

                let telegram_message = TelegramMessage {
                    chat_id: telegram_chat_id,
                    text: formatted_news,
                    parse_mode: "HTML".to_string(),
                };

                let post = reqwest::Client::new()
                    .post(telegram_send_message_url)
                    .json(&telegram_message)
                    .send()
                    .await?;

                let post_response_text = post.text().await?;

                println!("post response: {:?}", post_response_text);

                Ok(post_response_text)
            });

        let responses = try_join_all(n).await?;

        for response in responses {
            println!("post response: {:?}", response);
        }

        // for x in n {
        //     let response = x.await?;
        //     println!("post response: {:?}", response);
        // }

        "".to_string()
    };

    //println!("{}", Table::new(news));
    Ok(())
}
