use crate::app_data::rss_news::news_item::NewsItem;
use crate::formatting_text::clean_text::clean_title;
use anyhow::Context;
use chrono::Local;
use reqwest::Url;
use sled::Db;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

pub struct NewsReadTodayDb {
    db: Db,
}

impl NewsReadTodayDb {
    pub fn open_news_read_today_db() -> anyhow::Result<Self> {
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(".local_db")
            .join("news_read_v2");

        if let Some(parent_path) = db_path.parent() {
            fs::create_dir_all(parent_path).context("Could not create local db folder")?;
        }

        let db = sled::open(&db_path).context("Could not open local news-read db")?;

        let instance = Self { db };
        let _ = instance.cleanup_old_news();

        Ok(instance)
    }

    fn cleanup_old_news(&self) -> anyhow::Result<()> {
        let cutoff_time = Local::now().timestamp() - (7 * 24 * 60 * 60);
        for entry in self.db.iter() {
            if let Ok((key, value)) = entry {
                if value.len() == 8 {
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(&value);
                    let timestamp = i64::from_be_bytes(bytes);
                    if timestamp < cutoff_time {
                        let _ = self.db.remove(key);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn filter_out_news_already_read_today(
        &self,
        news: Vec<NewsItem>,
    ) -> anyhow::Result<Vec<NewsItem>> {
        let mut fresh_news = Vec::new();
        let mut keys_seen_in_this_run: HashSet<String> = HashSet::new();

        for item in news {
            let keys = news_read_today_keys(&item);
            let was_read_before = self.has_any_news_read_today_key(&keys)?;
            let was_seen_in_this_run = keys.iter().any(|key| keys_seen_in_this_run.contains(key));

            if was_read_before || was_seen_in_this_run {
                continue;
            }

            keys_seen_in_this_run.extend(keys);
            fresh_news.push(item);
        }

        Ok(fresh_news)
    }

    pub fn save_news_read_today(&self, news: &[NewsItem]) -> anyhow::Result<()> {
        let now_bytes = Local::now().timestamp().to_be_bytes();
        for item in news {
            for key in news_read_today_keys(item) {
                self.db
                    .insert(key.as_bytes(), &now_bytes)
                    .with_context(|| format!("Could not save news-read key: {key}"))?;
            }
        }

        self.db
            .flush()
            .context("Could not flush local news-read db")?;

        Ok(())
    }

    fn has_any_news_read_today_key(&self, keys: &[String]) -> anyhow::Result<bool> {
        for key in keys {
            if self
                .db
                .contains_key(key.as_bytes())
                .with_context(|| format!("Could not check news-read key: {key}"))?
            {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

fn news_read_today_keys(item: &NewsItem) -> Vec<String> {
    let mut keys = Vec::new();
    let clean_title_text = clean_title(&item.title);
    let clean_source_text = item.source.trim().to_lowercase();

    if let Some(clean_link) = clean_link_for_news_read_today(&item.link) {
        keys.push(format!("link:{clean_link}"));
    }

    if !clean_title_text.is_empty() {
        keys.push(format!("title:{clean_title_text}"));
        keys.push(format!(
            "source_title:{clean_source_text}:{clean_title_text}"
        ));
    }

    keys.push(format!(
        "published:{}:{}:{}",
        clean_source_text,
        clean_title_text,
        item.published_at.trim().to_lowercase()
    ));

    keys
}

fn clean_link_for_news_read_today(link: &str) -> Option<String> {
    let trimmed_link = link.trim();
    if trimmed_link.is_empty() {
        return None;
    }

    let parsed = Url::parse(trimmed_link).ok()?;
    let host = parsed.host_str()?.to_lowercase();
    let path = parsed.path().trim_end_matches('/').to_lowercase();

    Some(format!("{host}{path}"))
}
