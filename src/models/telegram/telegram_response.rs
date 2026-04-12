use serde::Deserialize;
use serde::de::IgnoredAny;

#[derive(Deserialize)]
pub struct TelegramResponse {
    pub ok: bool,
    pub result: Option<IgnoredAny>,
    pub description: Option<String>,
    pub error_code: Option<i64>,
}
