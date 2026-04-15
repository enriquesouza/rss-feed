use std::sync::OnceLock;

pub static APP_ENV: OnceLock<AppEnv> = OnceLock::new();

pub static NEWS_RULES: std::sync::LazyLock<super::news_rules::NewsRules> =
    std::sync::LazyLock::new(|| {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let path = std::path::Path::new(&manifest_dir).join("src/prompts/news_rules.yml");
        let content =
            std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read {:?}", path));
        serde_norway::from_str(&content).expect("Failed to parse news_rules.yml")
    });

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
