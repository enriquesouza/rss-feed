use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TelegramMessage {
    pub chat_id: String,
    pub text: String,
    //parse_mode: String,
}
