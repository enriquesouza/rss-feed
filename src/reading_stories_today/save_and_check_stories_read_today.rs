use crate::app_data::news_group::NewsGroup;
use crate::formatting_text::clean_text::clean_title;
use crate::grouping_news::check_if_groups_match::{has_shared_important_word, shared_word_count};
use anyhow::Context;
use chrono::Local;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoryReadToday {
    group_name: String,
    topic_words: Vec<String>,
    clean_titles: Vec<String>,
    clean_links: Vec<String>,
}

pub struct StoriesReadTodayDb {
    db: Db,
}

impl StoriesReadTodayDb {
    pub fn open_stories_read_today_db() -> anyhow::Result<Self> {
        let today_folder_name = today_folder_name();
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(".local_db")
            .join("stories_read_today")
            .join(&today_folder_name);

        if let Some(parent_path) = db_path.parent() {
            fs::create_dir_all(parent_path).context("Could not create local db folder")?;
            clear_old_day_folders(parent_path, &today_folder_name)?;
        }

        let db = sled::open(&db_path).context("Could not open local stories-read-today db")?;

        Ok(Self { db })
    }

    pub fn filter_out_stories_already_read_today(
        &self,
        stories: Vec<NewsGroup>,
    ) -> anyhow::Result<Vec<NewsGroup>> {
        let mut saved_stories = self.load_saved_stories()?;
        let mut fresh_stories = Vec::new();

        for story in stories {
            let story_to_check = make_story_read_today(&story);
            let story_was_already_read = saved_stories
                .iter()
                .any(|saved_story| same_story_read_today(saved_story, &story_to_check));

            if story_was_already_read {
                continue;
            }

            saved_stories.push(story_to_check);
            fresh_stories.push(story);
        }

        Ok(fresh_stories)
    }

    pub fn save_stories_read_today(&self, stories: &[NewsGroup]) -> anyhow::Result<()> {
        for story in stories {
            let story_id = self
                .db
                .generate_id()
                .context("Could not create a local story id")?;
            let story_key = format!("story:{story_id:020}");
            let story_value = serde_json::to_vec(&make_story_read_today(story))
                .context("Could not turn story into local db text")?;

            self.db
                .insert(story_key.as_bytes(), story_value)
                .with_context(|| format!("Could not save story-read-today key: {story_key}"))?;
        }

        self.db
            .flush()
            .context("Could not flush local stories-read-today db")?;

        Ok(())
    }

    fn load_saved_stories(&self) -> anyhow::Result<Vec<StoryReadToday>> {
        let mut stories = Vec::new();

        for story_entry in self.db.scan_prefix("story:") {
            let (_, story_value) = story_entry.context("Could not read a local story entry")?;
            let story = serde_json::from_slice::<StoryReadToday>(&story_value)
                .context("Could not read story from local db text")?;
            stories.push(story);
        }

        Ok(stories)
    }
}

fn same_story_read_today(left: &StoryReadToday, right: &StoryReadToday) -> bool {
    if left.group_name != right.group_name {
        return false;
    }

    if left.clean_links.iter().any(|left_link| {
        right
            .clean_links
            .iter()
            .any(|right_link| right_link == left_link)
    }) {
        return true;
    }

    if left.clean_titles.iter().any(|left_title| {
        right
            .clean_titles
            .iter()
            .any(|right_title| right_title == left_title)
    }) {
        return true;
    }

    let shared_words = shared_word_count(&left.topic_words, &right.topic_words);
    if shared_words >= 2 {
        return true;
    }

    shared_words >= 1 && has_shared_important_word(&left.topic_words, &right.topic_words)
}

fn make_story_read_today(story: &NewsGroup) -> StoryReadToday {
    let mut topic_words = story.topic_words.clone();
    topic_words.sort();
    topic_words.dedup();

    let mut clean_titles = story
        .items
        .iter()
        .map(|item| clean_title(&item.title))
        .filter(|title| !title.is_empty())
        .collect::<Vec<_>>();
    clean_titles.sort();
    clean_titles.dedup();

    let mut clean_links = story
        .items
        .iter()
        .filter_map(|item| clean_link_for_story_read_today(&item.link))
        .collect::<Vec<_>>();
    clean_links.sort();
    clean_links.dedup();

    StoryReadToday {
        group_name: story.group_name.clone(),
        topic_words,
        clean_titles,
        clean_links,
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

fn clean_link_for_story_read_today(link: &str) -> Option<String> {
    let trimmed_link = link.trim();
    if trimmed_link.is_empty() {
        return None;
    }

    let parsed = Url::parse(trimmed_link).ok()?;
    let host = parsed.host_str()?.to_lowercase();
    let path = parsed.path().trim_end_matches('/').to_lowercase();

    Some(format!("{host}{path}"))
}
