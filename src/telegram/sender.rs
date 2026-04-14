use crate::models::configs::config::Env;
use crate::models::telegram::telegram_message::TelegramMessage;
use crate::models::telegram::telegram_response::TelegramResponse;
use crate::telegram::formatter::normalize_llm_output_for_telegram;
use std::error::Error;

pub async fn send_via_telegram2(
    client: &reqwest::Client,
    news: String,
) -> Result<TelegramResponse, Box<dyn Error>> {
    let mut response: TelegramResponse = Default::default();

    if !news.is_empty() {
        let formatted_news = normalize_llm_output_for_telegram(&news);
        let env = Env::new();

        let telegram_message = TelegramMessage {
            chat_id: env.telegram_chat_id.clone(),
            text: formatted_news,
        };

        let post = client
            .post(format!(
                "https://api.telegram.org/bot{}/sendMessage",
                env.telegram_bot_token
            ))
            .json(&telegram_message)
            .send()
            .await?;

        response = post.json().await?;
    }

    Ok(response)
}
