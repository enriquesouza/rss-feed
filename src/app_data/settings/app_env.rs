use std::sync::OnceLock;

pub static APP_ENV: OnceLock<AppEnv> = OnceLock::new();

#[derive(Clone, Default)]
pub struct AppEnv {
    pub open_router_api_key: String,
    pub telegram_chat_id: String,
    pub telegram_bot_token: String,
}

impl AppEnv {
    pub fn get() -> &'static Self {
        APP_ENV.get_or_init(|| Self {
            open_router_api_key: std::env::var("OPEN_ROUTER_API_KEY")
                .expect("OPEN_ROUTER_API_KEY not set"),
            telegram_bot_token: std::env::var("TELEGRAM_BOT_TOKEN")
                .expect("TELEGRAM_BOT_TOKEN not set"),
            telegram_chat_id: std::env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID not set"),
        })
    }
}
