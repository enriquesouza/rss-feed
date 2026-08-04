use anyhow::Context;
use chrono::Local;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

/// One post draft ready to be saved.
#[derive(Debug, Clone)]
pub struct NewPost {
    pub topic: String,
    pub text_en: String,
    pub quote_url: Option<String>,
    pub aizzy_text: Option<String>,
}

/// One post row as stored in SQLite.
#[derive(Debug, Clone)]
pub struct PostRow {
    pub id: i64,
    pub made_at: String,
    pub topic: String,
    pub text_en: String,
    pub final_text: Option<String>,
    pub quote_url: Option<String>,
    pub aizzy_text: Option<String>,
    pub status: String,
    pub x_url: Option<String>,
    pub posted_at: Option<String>,
}

/// One news item kept from a fetch run, so we can browse what the feeds
/// brought by subject and by day.
#[derive(Debug, Clone)]
pub struct NewsItemRow {
    pub run_at: String,
    pub topic: String,
    pub title: String,
    pub link: Option<String>,
    pub source: Option<String>,
}

/// One consolidated story of a run: the same fact seen across several feeds,
/// already summarized by the AI, with the importance score the pipeline gave it.
#[derive(Debug, Clone)]
pub struct NewsStory {
    pub id: i64,
    pub run_at: String,
    pub subject: String,
    pub headline: String,
    pub score: i64,
    pub item_count: i64,
    pub source_count: i64,
    pub digest: Option<String>,
}

/// A story plus the individual news items behind it.
#[derive(Debug, Clone)]
pub struct StoryWithItems {
    pub story: NewsStory,
    pub items: Vec<NewsItemRow>,
}

/// One metrics snapshot for a posted post.
#[derive(Debug, Clone)]
pub struct MetricsRow {
    pub id: i64,
    pub post_id: i64,
    pub taken_at: String,
    pub views: i64,
    pub likes: i64,
    pub reposts: i64,
    pub replies: i64,
    pub bookmarks: i64,
    pub followers_total: Option<i64>,
}

/// Keeps every generated post and its life cycle:
/// gerado -> aprovado -> postado (with the X link) -> metrics over time.
/// Also: descartado.
pub struct PostsDb {
    conn: Mutex<Connection>,
}

impl PostsDb {
    pub fn open_posts_db() -> anyhow::Result<Self> {
        let db_path = PostsDb::posts_db_path();

        if let Some(parent_path) = db_path.parent() {
            std::fs::create_dir_all(parent_path).context("Could not create local db folder")?;
        }

        let conn = Connection::open(&db_path).context("Could not open posts sqlite db")?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                made_at TEXT NOT NULL,
                topic TEXT NOT NULL,
                text_en TEXT NOT NULL,
                final_text TEXT,
                quote_url TEXT,
                aizzy_text TEXT,
                status TEXT NOT NULL DEFAULT 'gerado',
                x_url TEXT,
                posted_at TEXT
            );
            CREATE TABLE IF NOT EXISTS post_metrics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                post_id INTEGER NOT NULL REFERENCES posts(id),
                taken_at TEXT NOT NULL,
                views INTEGER NOT NULL DEFAULT 0,
                likes INTEGER NOT NULL DEFAULT 0,
                reposts INTEGER NOT NULL DEFAULT 0,
                replies INTEGER NOT NULL DEFAULT 0,
                bookmarks INTEGER NOT NULL DEFAULT 0,
                followers_total INTEGER
            );
            CREATE TABLE IF NOT EXISTS news_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                run_at TEXT NOT NULL,
                run_day TEXT NOT NULL,
                topic TEXT NOT NULL,
                title TEXT NOT NULL,
                link TEXT,
                source TEXT
            );
            CREATE INDEX IF NOT EXISTS news_day_idx ON news_items(run_day);
            CREATE TABLE IF NOT EXISTS news_stories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                run_at TEXT NOT NULL,
                run_day TEXT NOT NULL,
                subject TEXT NOT NULL,
                headline TEXT NOT NULL,
                score INTEGER NOT NULL DEFAULT 0,
                item_count INTEGER NOT NULL DEFAULT 0,
                source_count INTEGER NOT NULL DEFAULT 0,
                digest TEXT
            );
            CREATE INDEX IF NOT EXISTS story_day_idx ON news_stories(run_day);
            ",
        )
        .context("Could not create posts tables")?;

        // The items table gained a story link after the first release.
        let _ = conn.execute("ALTER TABLE news_items ADD COLUMN story_id INTEGER", []);
        // Posts remember which story and subject they came from.
        let _ = conn.execute("ALTER TABLE posts ADD COLUMN story_id INTEGER", []);
        let _ = conn.execute("ALTER TABLE posts ADD COLUMN subject TEXT", []);

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn posts_db_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(".local_db")
            .join("posts.sqlite")
    }

    fn now_text() -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn save_new_posts(&self, new_posts: &[NewPost]) -> anyhow::Result<Vec<i64>> {
        let conn = self.conn.lock().expect("posts db lock");
        let made_at = Self::now_text();
        let mut ids = Vec::with_capacity(new_posts.len());
        for post in new_posts {
            conn.execute(
                "INSERT INTO posts (made_at, topic, text_en, quote_url, aizzy_text, status)
                 VALUES (?1, ?2, ?3, ?4, ?5, 'gerado')",
                rusqlite::params![
                    made_at,
                    post.topic,
                    post.text_en,
                    post.quote_url,
                    post.aizzy_text
                ],
            )
            .context("Could not insert post")?;
            ids.push(conn.last_insert_rowid());
        }
        Ok(ids)
    }

    /// Save one draft that came from a news story, so the portal can show
    /// "já virou post" and the analytics can group by subject.
    pub fn save_post_from_story(
        &self,
        story_id: i64,
        subject: &str,
        topic: &str,
        text_en: &str,
    ) -> anyhow::Result<i64> {
        let conn = self.conn.lock().expect("posts db lock");
        conn.execute(
            "INSERT INTO posts (made_at, topic, text_en, status, story_id, subject)
             VALUES (?1, ?2, ?3, 'gerado', ?4, ?5)",
            rusqlite::params![Self::now_text(), topic, text_en, story_id, subject],
        )
        .context("Could not insert post from story")?;
        Ok(conn.last_insert_rowid())
    }

    /// How many drafts each story already produced.
    pub fn count_posts_by_story(&self) -> anyhow::Result<Vec<(i64, i64)>> {
        let conn = self.conn.lock().expect("posts db lock");
        let mut stmt = conn.prepare(
            "SELECT story_id, COUNT(*) FROM posts WHERE story_id IS NOT NULL GROUP BY story_id",
        )?;
        let rows = stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)))?;
        let mut counts = Vec::new();
        for row in rows {
            counts.push(row?);
        }
        Ok(counts)
    }

    /// Posts of one subject — the analytics view when you arrive from a story.
    pub fn list_posts_of_subject(&self, subject: &str) -> anyhow::Result<Vec<PostRow>> {
        let conn = self.conn.lock().expect("posts db lock");
        let mut stmt = conn.prepare(
            "SELECT id, made_at, topic, text_en, final_text, quote_url, aizzy_text,
             status, x_url, posted_at FROM posts WHERE subject = ?1 ORDER BY id DESC LIMIT 200",
        )?;
        let rows = stmt.query_map([subject], Self::row_to_post)?;
        let mut posts = Vec::new();
        for row in rows {
            posts.push(row?);
        }
        Ok(posts)
    }

    /// One story by id, with its items.
    pub fn get_story(&self, story_id: i64) -> anyhow::Result<Option<StoryWithItems>> {
        let conn = self.conn.lock().expect("posts db lock");
        let mut stmt = conn.prepare(
            "SELECT id, run_at, subject, headline, score, item_count, source_count, digest
             FROM news_stories WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map([story_id], |row| {
            Ok(NewsStory {
                id: row.get(0)?,
                run_at: row.get(1)?,
                subject: row.get(2)?,
                headline: row.get(3)?,
                score: row.get(4)?,
                item_count: row.get(5)?,
                source_count: row.get(6)?,
                digest: row.get(7)?,
            })
        })?;
        let Some(story) = rows.next().transpose()? else {
            return Ok(None);
        };
        drop(rows);
        drop(stmt);

        let mut items_stmt = conn.prepare(
            "SELECT run_at, topic, title, link, source FROM news_items
             WHERE story_id = ?1 ORDER BY id ASC",
        )?;
        let item_rows = items_stmt.query_map([story_id], |row| {
            Ok(NewsItemRow {
                run_at: row.get(0)?,
                topic: row.get(1)?,
                title: row.get(2)?,
                link: row.get(3)?,
                source: row.get(4)?,
            })
        })?;
        let mut items = Vec::new();
        for item in item_rows {
            items.push(item?);
        }
        Ok(Some(StoryWithItems { story, items }))
    }

    fn row_to_post(row: &rusqlite::Row<'_>) -> rusqlite::Result<PostRow> {
        Ok(PostRow {
            id: row.get(0)?,
            made_at: row.get(1)?,
            topic: row.get(2)?,
            text_en: row.get(3)?,
            final_text: row.get(4)?,
            quote_url: row.get(5)?,
            aizzy_text: row.get(6)?,
            status: row.get(7)?,
            x_url: row.get(8)?,
            posted_at: row.get(9)?,
        })
    }

    pub fn list_posts(&self, status_filter: Option<&str>) -> anyhow::Result<Vec<PostRow>> {
        let conn = self.conn.lock().expect("posts db lock");
        let base = "SELECT id, made_at, topic, text_en, final_text, quote_url, aizzy_text,
                    status, x_url, posted_at FROM posts";
        let mut posts = Vec::new();
        match status_filter {
            Some(status) => {
                let sql = format!("{base} WHERE status = ?1 ORDER BY id DESC LIMIT 200");
                let mut stmt = conn.prepare(&sql)?;
                let rows = stmt.query_map([status], Self::row_to_post)?;
                for row in rows {
                    posts.push(row?);
                }
            }
            None => {
                let sql = format!("{base} ORDER BY id DESC LIMIT 200");
                let mut stmt = conn.prepare(&sql)?;
                let rows = stmt.query_map([], Self::row_to_post)?;
                for row in rows {
                    posts.push(row?);
                }
            }
        }
        Ok(posts)
    }

    pub fn get_post(&self, post_id: i64) -> anyhow::Result<Option<PostRow>> {
        let conn = self.conn.lock().expect("posts db lock");
        let mut stmt = conn.prepare(
            "SELECT id, made_at, topic, text_en, final_text, quote_url, aizzy_text,
             status, x_url, posted_at FROM posts WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map([post_id], Self::row_to_post)?;
        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    pub fn set_post_status(&self, post_id: i64, status: &str) -> anyhow::Result<()> {
        let conn = self.conn.lock().expect("posts db lock");
        conn.execute(
            "UPDATE posts SET status = ?1 WHERE id = ?2",
            rusqlite::params![status, post_id],
        )
        .context("Could not update post status")?;
        Ok(())
    }

    pub fn set_final_text(&self, post_id: i64, final_text: &str) -> anyhow::Result<()> {
        let conn = self.conn.lock().expect("posts db lock");
        conn.execute(
            "UPDATE posts SET final_text = ?1 WHERE id = ?2",
            rusqlite::params![final_text, post_id],
        )
        .context("Could not update post text")?;
        Ok(())
    }

    pub fn mark_post_posted(&self, post_id: i64, x_url: &str) -> anyhow::Result<()> {
        let conn = self.conn.lock().expect("posts db lock");
        conn.execute(
            "UPDATE posts SET status = 'postado', x_url = ?1, posted_at = ?2 WHERE id = ?3",
            rusqlite::params![x_url, Self::now_text(), post_id],
        )
        .context("Could not mark post as posted")?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_post_metrics(
        &self,
        post_id: i64,
        views: i64,
        likes: i64,
        reposts: i64,
        replies: i64,
        bookmarks: i64,
        followers_total: Option<i64>,
    ) -> anyhow::Result<()> {
        let conn = self.conn.lock().expect("posts db lock");
        conn.execute(
            "INSERT INTO post_metrics
             (post_id, taken_at, views, likes, reposts, replies, bookmarks, followers_total)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                post_id,
                Self::now_text(),
                views,
                likes,
                reposts,
                replies,
                bookmarks,
                followers_total
            ],
        )
        .context("Could not insert post metrics")?;
        Ok(())
    }

    pub fn list_post_metrics(&self, post_id: i64) -> anyhow::Result<Vec<MetricsRow>> {
        let conn = self.conn.lock().expect("posts db lock");
        let mut stmt = conn.prepare(
            "SELECT id, post_id, taken_at, views, likes, reposts, replies, bookmarks,
             followers_total FROM post_metrics WHERE post_id = ?1 ORDER BY id ASC",
        )?;
        let rows = stmt.query_map([post_id], |row| {
            Ok(MetricsRow {
                id: row.get(0)?,
                post_id: row.get(1)?,
                taken_at: row.get(2)?,
                views: row.get(3)?,
                likes: row.get(4)?,
                reposts: row.get(5)?,
                replies: row.get(6)?,
                bookmarks: row.get(7)?,
                followers_total: row.get(8)?,
            })
        })?;
        let mut metrics = Vec::new();
        for row in rows {
            metrics.push(row?);
        }
        Ok(metrics)
    }

    /// Latest metrics snapshot for each posted post — the analytics base.
    pub fn list_latest_metrics(&self) -> anyhow::Result<Vec<(PostRow, Option<MetricsRow>)>> {
        let posted = self.list_posts(Some("postado"))?;
        let mut out = Vec::with_capacity(posted.len());
        for post in posted {
            let latest = self.list_post_metrics(post.id)?.into_iter().last();
            out.push((post, latest));
        }
        Ok(out)
    }

    /// Save one consolidated story and the items behind it, in a single run stamp.
    /// `run_at` is shared by every story of the same cycle so the reader can see
    /// "1ª rodada", "2ª rodada" of the day.
    pub fn save_news_story(
        &self,
        run_at: &str,
        subject: &str,
        headline: &str,
        score: i64,
        digest: Option<&str>,
        items: &[NewsItemRow],
    ) -> anyhow::Result<i64> {
        let conn = self.conn.lock().expect("posts db lock");
        let run_day = run_at.get(..10).unwrap_or(run_at).to_string();

        let mut sources: Vec<&str> = items
            .iter()
            .filter_map(|item| item.source.as_deref())
            .collect();
        sources.sort_unstable();
        sources.dedup();

        conn.execute(
            "INSERT INTO news_stories
             (run_at, run_day, subject, headline, score, item_count, source_count, digest)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                run_at,
                run_day,
                subject,
                headline,
                score,
                items.len() as i64,
                sources.len() as i64,
                digest
            ],
        )
        .context("Could not insert news story")?;
        let story_id = conn.last_insert_rowid();

        for item in items {
            conn.execute(
                "INSERT INTO news_items (run_at, run_day, topic, title, link, source, story_id)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                rusqlite::params![
                    run_at,
                    run_day,
                    subject,
                    item.title,
                    item.link,
                    item.source,
                    story_id
                ],
            )
            .context("Could not insert news item")?;
        }

        Ok(story_id)
    }

    /// Stories of one day with their items, strongest first.
    /// Pass `None` for the most recent day that has stories.
    pub fn list_stories_of_day(&self, day: Option<&str>) -> anyhow::Result<Vec<StoryWithItems>> {
        let conn = self.conn.lock().expect("posts db lock");
        let day_wanted: String = match day {
            Some(d) => d.to_string(),
            None => conn
                .query_row(
                    "SELECT run_day FROM news_stories ORDER BY run_day DESC LIMIT 1",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or_default(),
        };
        if day_wanted.is_empty() {
            return Ok(Vec::new());
        }

        let mut stmt = conn.prepare(
            "SELECT id, run_at, subject, headline, score, item_count, source_count, digest
             FROM news_stories WHERE run_day = ?1
             ORDER BY run_at DESC, score DESC",
        )?;
        let rows = stmt.query_map([&day_wanted], |row| {
            Ok(NewsStory {
                id: row.get(0)?,
                run_at: row.get(1)?,
                subject: row.get(2)?,
                headline: row.get(3)?,
                score: row.get(4)?,
                item_count: row.get(5)?,
                source_count: row.get(6)?,
                digest: row.get(7)?,
            })
        })?;

        let mut stories = Vec::new();
        for row in rows {
            stories.push(row?);
        }

        let mut items_stmt = conn.prepare(
            "SELECT run_at, topic, title, link, source FROM news_items
             WHERE story_id = ?1 ORDER BY id ASC",
        )?;
        let mut out = Vec::with_capacity(stories.len());
        for story in stories {
            let item_rows = items_stmt.query_map([story.id], |row| {
                Ok(NewsItemRow {
                    run_at: row.get(0)?,
                    topic: row.get(1)?,
                    title: row.get(2)?,
                    link: row.get(3)?,
                    source: row.get(4)?,
                })
            })?;
            let mut items = Vec::new();
            for item in item_rows {
                items.push(item?);
            }
            out.push(StoryWithItems { story, items });
        }
        Ok(out)
    }

    /// Every day that has stories, newest first, with how many stories it holds.
    pub fn list_story_days(&self) -> anyhow::Result<Vec<(String, i64)>> {
        let conn = self.conn.lock().expect("posts db lock");
        let mut stmt = conn.prepare(
            "SELECT run_day, COUNT(*) FROM news_stories GROUP BY run_day ORDER BY run_day DESC LIMIT 60",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;
        let mut days = Vec::new();
        for row in rows {
            days.push(row?);
        }
        Ok(days)
    }

    /// Save every news item of one fetch run, stamped with the same run time.
    pub fn save_news_run(&self, items: &[NewsItemRow]) -> anyhow::Result<()> {
        let conn = self.conn.lock().expect("posts db lock");
        let run_at = Self::now_text();
        let run_day = run_at[..10].to_string();
        for item in items {
            conn.execute(
                "INSERT INTO news_items (run_at, run_day, topic, title, link, source)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![
                    run_at,
                    run_day,
                    item.topic,
                    item.title,
                    item.link,
                    item.source
                ],
            )
            .context("Could not insert news item")?;
        }
        Ok(())
    }

    /// News of one day, newest run first, ready to be grouped by subject.
    /// Pass `None` for the most recent day that has news.
    pub fn list_news_of_day(&self, day: Option<&str>) -> anyhow::Result<Vec<NewsItemRow>> {
        let conn = self.conn.lock().expect("posts db lock");
        let day_wanted: String = match day {
            Some(d) => d.to_string(),
            None => conn
                .query_row(
                    "SELECT run_day FROM news_items ORDER BY run_day DESC LIMIT 1",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or_default(),
        };
        if day_wanted.is_empty() {
            return Ok(Vec::new());
        }

        let mut stmt = conn.prepare(
            "SELECT run_at, topic, title, link, source FROM news_items
             WHERE run_day = ?1 ORDER BY run_at DESC, topic ASC, id ASC",
        )?;
        let rows = stmt.query_map([day_wanted], |row| {
            Ok(NewsItemRow {
                run_at: row.get(0)?,
                topic: row.get(1)?,
                title: row.get(2)?,
                link: row.get(3)?,
                source: row.get(4)?,
            })
        })?;
        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }

    /// Every day that has news, newest first, with how many items it holds —
    /// this is what the calendar draws.
    pub fn list_news_days(&self) -> anyhow::Result<Vec<(String, i64)>> {
        let conn = self.conn.lock().expect("posts db lock");
        let mut stmt = conn.prepare(
            "SELECT run_day, COUNT(*) FROM news_items GROUP BY run_day ORDER BY run_day DESC LIMIT 60",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;
        let mut days = Vec::new();
        for row in rows {
            days.push(row?);
        }
        Ok(days)
    }

    pub fn count_posts_by_status(&self) -> anyhow::Result<Vec<(String, i64)>> {
        let conn = self.conn.lock().expect("posts db lock");
        let mut stmt =
            conn.prepare("SELECT status, COUNT(*) FROM posts GROUP BY status ORDER BY status")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;
        let mut counts = Vec::new();
        for row in rows {
            counts.push(row?);
        }
        Ok(counts)
    }
}
