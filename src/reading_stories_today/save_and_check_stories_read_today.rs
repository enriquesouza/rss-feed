use crate::app_data::{
    news_group::NewsGroup,
    ollama::{NLU_MODEL, OllamaClient},
    open_router::ChatMessage,
    open_router::chat_message::MessageContent,
};
use crate::formatting_text::clean_text::clean_title;
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
    #[serde(default)]
    added_at: i64,
}

pub struct StoriesReadTodayDb {
    db: Db,
}

#[derive(serde::Deserialize)]
struct NluResponse {
    duplicate: bool,
}

impl StoriesReadTodayDb {
    pub fn open_stories_read_today_db() -> anyhow::Result<Self> {
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(".local_db")
            .join("stories_read_v2");

        if let Some(parent_path) = db_path.parent() {
            fs::create_dir_all(parent_path).context("Could not create local db folder")?;
        }

        let db = sled::open(&db_path).context("Could not open local stories-read db")?;

        Ok(Self { db })
    }

    pub async fn filter_out_stories_already_read_today<'a>(
        &self,
        client: &reqwest::Client,
        stories: Vec<NewsGroup>,
    ) -> anyhow::Result<Vec<NewsGroup>> {
        let ollama_host = crate::app_data::settings::app_env::AppEnv::get()
            .ollama_host
            .clone();
        let ollama = OllamaClient::new(client, ollama_host);

        let mut saved_stories = self.load_saved_stories()?;
        let mut fresh_stories = Vec::new();

        for story in stories {
            let story_to_check = make_story_read_today(&story);

            // 1. Fast path: exact link or exact title intersection
            let mut is_duplicate = saved_stories
                .iter()
                .any(|saved_story| is_exact_match(saved_story, &story_to_check));

            // 2. Slow path: Semantic NLU check if not an exact match
            if !is_duplicate {
                let relevant_past: Vec<_> = saved_stories
                    .iter()
                    .filter(|s| s.group_name == story_to_check.group_name)
                    .collect();

                if !relevant_past.is_empty() {
                    match check_duplicate_with_ai(&ollama, &story_to_check, &relevant_past).await {
                        Ok(true) => {
                            println!("NLU deduplicated story: {:?}", story_to_check.clean_titles);
                            is_duplicate = true;
                        }
                        Ok(false) => {
                            // It is truly unique
                        }
                        Err(e) => {
                            eprintln!("NLU check failed: {}. Permitting story to pass.", e);
                        }
                    }
                }
            }

            if is_duplicate {
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
                .with_context(|| format!("Could not save story-read key: {story_key}"))?;
        }

        self.db
            .flush()
            .context("Could not flush local stories-read db")?;

        Ok(())
    }

    fn load_saved_stories(&self) -> anyhow::Result<Vec<StoryReadToday>> {
        let mut stories = Vec::new();
        let cutoff_time = Local::now().timestamp() - (7 * 24 * 60 * 60);

        for story_entry in self.db.scan_prefix("story:") {
            let (story_key, story_value) =
                story_entry.context("Could not read a local story entry")?;
            let story = serde_json::from_slice::<StoryReadToday>(&story_value)
                .context("Could not read story from local db text")?;

            if story.added_at > 0 && story.added_at < cutoff_time {
                let _ = self.db.remove(story_key);
            } else {
                stories.push(story);
            }
        }

        Ok(stories)
    }
}

async fn check_duplicate_with_ai(
    ollama: &OllamaClient<'_>,
    new_story: &StoryReadToday,
    past_stories: &[&StoryReadToday],
) -> anyhow::Result<bool> {
    let mut past_texts = String::new();
    for (i, p) in past_stories.iter().enumerate() {
        past_texts.push_str(&format!("[P{}]\n", i));
        past_texts.push_str(&format!("Títulos: {}\n\n", p.clean_titles.join(" | ")));
    }

    let new_titles = new_story.clean_titles.join(" | ");

    let prompt = format!(
        "Temos as seguintes notícias que já foram publicadas nos últimos 7 dias:\n\n\
        {}\n\
        Agora, recebemos esta NOVA notícia:\n\
        Títulos: {}\n\n\
        Esta nova notícia se refere exatamente ao MESMO evento/ocorrência central de alguma das notícias antigas?\n\
        Se for o mesmo hack, o mesmo milestone de preço, a mesma parceria ou a mesma atualização, responda true.\n\
        Responda APENAS com um JSON estrito: {{ \"duplicate\": true }} ou {{ \"duplicate\": false }}",
        past_texts, new_titles
    );

    let messages = vec![ChatMessage {
        role: "user".to_string(),
        content: Some(MessageContent::Text(prompt)),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }];

    // Use low temperature for deterministic NLU, larger max_tokens so thinking doesn't get cut off
    let response = ollama
        .chat_completion(NLU_MODEL, messages, 0.0, 1000, Some("low"))
        .await?;

    let content = response
        .choices
        .first()
        .and_then(|item| item.message.text_content())
        .unwrap_or_default();

    println!("{}", &content);

    let result: NluResponse = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse NLU response '{}': {}", content, e))?;
    Ok(result.duplicate)
}

fn is_exact_match(left: &StoryReadToday, right: &StoryReadToday) -> bool {
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

    false
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
        added_at: Local::now().timestamp(),
    }
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
