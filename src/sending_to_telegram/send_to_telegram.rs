use crate::app_data::settings::app_env::AppEnv;
use crate::app_data::telegram::telegram_message::TelegramMessage;
use crate::app_data::telegram::telegram_response::TelegramResponse;
use crate::sending_to_telegram::format_text_for_telegram::format_text_for_telegram;
use std::error::Error;
use std::time::Duration;

const TELEGRAM_TIMEOUT_SECS: u64 = 15;

pub async fn send_to_telegram(
    client: &reqwest::Client,
    text: String,
) -> Result<TelegramResponse, Box<dyn Error>> {
    let mut telegram_response: TelegramResponse = Default::default();

    if !text.is_empty() {
        let message_text = format_text_for_telegram(&text);
        let env = AppEnv::get();

        let telegram_message = TelegramMessage {
            chat_id: env.telegram_chat_id.clone(),
            text: message_text,
        };

        let http_response = client
            .post(format!(
                "https://api.telegram.org/bot{}/sendMessage",
                env.telegram_bot_token
            ))
            .timeout(Duration::from_secs(TELEGRAM_TIMEOUT_SECS))
            .json(&telegram_message)
            .send()
            .await?;

        telegram_response = http_response.json().await?;
    }

    Ok(telegram_response)
}
