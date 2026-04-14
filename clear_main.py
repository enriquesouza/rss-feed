import re

with open("src/main.rs", "r") as f:
    text = f.read()

# Delete everything from `async fn get_rss_news` to the end of the file.
# Except tests
match = re.search(r"^async fn get_rss_news\(", text, re.MULTILINE)
if match:
    # also remove struct TopicCluster {
    text = re.sub(r"#\[derive\(Debug, Clone\)\]\nstruct TopicCluster \{.*?\n\}\n", "", text, flags=re.DOTALL)
    text = text[:match.start()]

new_uses = """
pub mod models;
pub mod services;
pub mod prompts;
pub mod curation;
pub mod clustering;
pub mod rss;
pub mod telegram;
pub mod formatters;

use crate::curation::engine::curate_news_for_llm;
use crate::clustering::engine::cluster_news_for_llm;
use crate::clustering::formatter::format_topic_cluster_for_llm;
use crate::rss::fetch::get_rss_news;
use crate::telegram::sender::send_via_telegram2;
"""

text = text.replace("use rss_feed::models::configs::config::CURATION_CONFIG;", "")
text = text.replace("use rss_feed::models::configs::config::Config;", "")
text = text.replace("use rss_feed::models::telegram::telegram_message::TelegramMessage;", "")
text = text.replace("use rss_feed::models::telegram::telegram_response::TelegramResponse;", "")
text = text.replace("use rss_feed::services::open_router_service::OpenRouterService;", "use crate::services::open_router_service::OpenRouterService;")
text = text.replace("use rss_feed::models::rss::channel_row::ChannelRow;", "use crate::models::rss::channel_row::ChannelRow;")

new_imports = "use std::sync::LazyLock;\n" + new_uses
text = text.replace("use std::sync::LazyLock;", new_imports)

# Remove unused consts from main
text = re.sub(r"const MAX_ITEMS_FOR_LLM.*?;\n", "", text)
text = re.sub(r"const TARGET_TECHNICAL_ITEMS.*?;\n", "", text)
text = re.sub(r"const MAX_TOPIC_CLUSTERS_FOR_LLM.*?;\n", "", text)
text = re.sub(r"const MAX_ARTICLES_PER_TOPIC_CLUSTER.*?;\n", "", text)

with open("src/main.rs", "w") as f:
    f.write(text)
