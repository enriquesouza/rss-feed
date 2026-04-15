use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct ResponseParameters {
    pub retry_after: Option<u32>,
    pub migrate_to_chat_id: Option<i64>,
}

#[derive(Deserialize, Default, Debug)]
pub struct SentTelegramMessage {
    pub message_id: i64,
    pub date: i64,
}

#[derive(Deserialize, Default, Debug)]
pub struct TelegramResponse {
    pub ok: bool,
    pub result: Option<SentTelegramMessage>,
    pub description: Option<String>,
    pub error_code: Option<i64>,
    pub parameters: Option<ResponseParameters>,
}
