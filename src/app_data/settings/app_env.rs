use std::sync::OnceLock;

pub static APP_ENV: OnceLock<AppEnv> = OnceLock::new();

#[derive(Clone, Default)]
pub struct AppEnv {
    pub omlx_host: String,
    pub omlx_api_key: Option<String>,
    pub telegram_chat_id: String,
    pub telegram_bot_token: String,
}

impl AppEnv {
    /// The pipeline cannot deliver anything without Telegram — say so loudly,
    /// at startup, instead of failing on the first send hours later.
    pub fn check_telegram_is_set() -> anyhow::Result<()> {
        let env = Self::get();
        if env.telegram_bot_token.is_empty() || env.telegram_chat_id.is_empty() {
            anyhow::bail!("TELEGRAM_BOT_TOKEN and TELEGRAM_CHAT_ID must be set in .env");
        }
        Ok(())
    }

    pub fn get() -> &'static Self {
        APP_ENV.get_or_init(|| Self {
            omlx_host: std::env::var("OMLX_HOST")
                .unwrap_or_else(|_| "http://localhost:8000".to_string()),
            omlx_api_key: std::env::var("OMLX_API_KEY").ok(),
            // Empty when unset: only the pipeline needs Telegram, and it checks
            // for it at startup (see check_telegram_is_set). The admin binary
            // must not die because a variable it never uses is missing.
            telegram_bot_token: std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default(),
            telegram_chat_id: std::env::var("TELEGRAM_CHAT_ID").unwrap_or_default(),
        })
    }
}
