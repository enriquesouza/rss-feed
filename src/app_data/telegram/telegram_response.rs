use serde::Deserialize;

use crate::app_data::telegram::{
    response_parameters::ResponseParameters, sent_telegram_message::SentTelegramMessage,
};

#[derive(Deserialize, Default, Debug)]
pub struct TelegramResponse {
    pub ok: bool,
    pub result: Option<SentTelegramMessage>,
    pub description: Option<String>,
    pub error_code: Option<i64>,
    pub parameters: Option<ResponseParameters>,
}
