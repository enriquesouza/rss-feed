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
        let today_folder_name = today_folder_name();
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(".local_db")
            .join("news_read_today")
            .join(&today_folder_name);

        if let Some(parent_path) = db_path.parent() {
            fs::create_dir_all(parent_path).context("Could not create local db folder")?;
            clear_old_day_folders(parent_path, &today_folder_name)?;
        }

        let db = sled::open(&db_path).context("Could not open local news-read-today db")?;

        Ok(Self { db })
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
        for item in news {
            for key in news_read_today_keys(item) {
                self.db
                    .insert(key.as_bytes(), &[1])
                    .with_context(|| format!("Could not save news-read-today key: {key}"))?;
            }
        }

        self.db
            .flush()
            .context("Could not flush local news-read-today db")?;

        Ok(())
    }

    fn has_any_news_read_today_key(&self, keys: &[String]) -> anyhow::Result<bool> {
        for key in keys {
            if self
                .db
                .contains_key(key.as_bytes())
                .with_context(|| format!("Could not check news-read-today key: {key}"))?
            {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

fn clear_old_day_folders(
    db_folder: &std::path::Path,
    today_folder_name: &str,
) -> anyhow::Result<()> {
    for folder_entry in fs::read_dir(db_folder).context("Could not read local db folder")? {
        let folder_entry = folder_entry.context("Could not read a local db folder entry")?;
        let folder_path = folder_entry.path();
        let folder_name = folder_entry.file_name().to_string_lossy().to_string();

        if folder_name != today_folder_name && folder_path.is_dir() {
            fs::remove_dir_all(&folder_path)
                .with_context(|| format!("Could not remove old local db folder: {folder_name}"))?;
        }
    }

    Ok(())
}

fn today_folder_name() -> String {
    Local::now().date_naive().format("%Y-%m-%d").to_string()
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
