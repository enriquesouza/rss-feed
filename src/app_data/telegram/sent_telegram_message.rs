use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct SentTelegramMessage {
    pub message_id: i64,
    pub date: i64,
}
