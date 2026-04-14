use std::sync::OnceLock;

#[derive(Clone)]
pub struct Config {
    pub telegram_chat_id: String,
    pub telegram_send_message_url: String,
}

pub static ENV: OnceLock<Env> = OnceLock::new();

pub static CURATION_CONFIG: std::sync::LazyLock<super::curation::CurationConfig> =
    std::sync::LazyLock::new(|| {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let path = std::path::Path::new(&manifest_dir).join("src/prompts/curation.yml");
        let content =
            std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read {:?}", path));
        serde_yml::from_str(&content).expect("Failed to parse curation.yml")
    });

#[derive(Clone, Default)]
pub struct Env {
    pub open_router_api_key: String,
    pub telegram_chat_id: String,
    pub telegram_bot_token: String,
}

impl Env {
    pub fn new() -> &'static Self {
        ENV.get_or_init(|| Self {
            open_router_api_key: std::env::var("OPEN_ROUTER_API_KEY")
                .expect("OPEN_ROUTER_API_KEY not set"),
            telegram_bot_token: std::env::var("TELEGRAM_BOT_TOKEN")
                .expect("TELEGRAM_BOT_TOKEN not set"),
            telegram_chat_id: std::env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID not set"),
        })
    }
}
