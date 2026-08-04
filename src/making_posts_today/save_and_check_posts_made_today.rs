use anyhow::Context;
use chrono::Local;
use sled::Db;
use std::fs;
use std::path::PathBuf;

/// Remembers if we already made the X post drafts for today,
/// so the loop makes them only once per day.
pub struct PostsMadeTodayDb {
    db: Db,
}

impl PostsMadeTodayDb {
    pub fn open_posts_made_today_db() -> anyhow::Result<Self> {
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(".local_db")
            .join("posts_made_v1");

        if let Some(parent_path) = db_path.parent() {
            fs::create_dir_all(parent_path).context("Could not create local db folder")?;
        }

        let db = sled::open(&db_path).context("Could not open local posts-made db")?;

        Ok(Self { db })
    }

    fn today_key() -> String {
        format!("posts_{}", Local::now().format("%Y-%m-%d"))
    }

    pub fn check_posts_made_today(&self) -> anyhow::Result<bool> {
        let key = Self::today_key();
        let found = self
            .db
            .get(key.as_bytes())
            .context("Could not read posts-made db")?;
        Ok(found.is_some())
    }

    pub fn save_posts_made_today(&self) -> anyhow::Result<()> {
        let key = Self::today_key();
        self.db
            .insert(key.as_bytes(), b"done")
            .context("Could not write posts-made db")?;
        self.db.flush().context("Could not flush posts-made db")?;
        Ok(())
    }
}
