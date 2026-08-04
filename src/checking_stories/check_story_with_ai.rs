use crate::app_data::news_group::NewsGroup;
use crate::app_data::omlx::{OmlxClient, STORY_CHECK_MODEL};
use crate::app_data::open_router::ChatMessage;
use crate::app_data::open_router::chat_message::MessageContent;
use crate::app_data::settings::app_env::AppEnv;
use crate::checking_stories::make_check_prompt::make_check_prompt;
use crate::checking_stories::read_group_answer::read_group_answer;
use crate::checking_stories::split_group_by_answer::split_group_by_answer;
use crate::grouping_news::score_news_group::score_news_group;

const MAX_ITEMS_TO_CHECK: usize = 12;
const CHECK_MAX_TOKENS: i32 = 1200;

/// Asks the AI whether a heuristic group really is one story and splits it if not.
/// Fail-open by design: any error keeps the original group — news is never lost.
pub struct StoryChecker<'a> {
    omlx: OmlxClient<'a>,
}

impl<'a> StoryChecker<'a> {
    pub fn new(client: &'a reqwest::Client) -> Self {
        let base_url = AppEnv::get().omlx_host.clone();
        let api_key = AppEnv::get().omlx_api_key.clone();
        Self {
            omlx: OmlxClient::new(client, base_url, api_key),
        }
    }

    /// Check every group one by one (sequential to keep request pressure low)
    /// and return the checked list sorted by score, best first.
    pub async fn check_stories(&self, groups: Vec<NewsGroup>) -> Vec<NewsGroup> {
        let mut checked_groups: Vec<NewsGroup> = Vec::new();
        for group in groups {
            checked_groups.extend(self.check_one_group(group).await);
        }
        checked_groups.sort_by_key(|group| std::cmp::Reverse(score_news_group(group)));
        checked_groups
    }

    async fn check_one_group(&self, group: NewsGroup) -> Vec<NewsGroup> {
        if group.items.len() < 2 {
            return vec![group]; // one item can never be a mixed story
        }
        if group.items.len() > MAX_ITEMS_TO_CHECK {
            eprintln!(
                "Story check skipped for '{}': {} items is more than {}",
                group.group_name,
                group.items.len(),
                MAX_ITEMS_TO_CHECK
            );
            return vec![group];
        }

        let titles: Vec<String> = group.items.iter().map(|item| item.title.clone()).collect();
        let prompt = make_check_prompt(&titles);

        // Attempt 1 at temperature 0.0; a retry at 0.2 because temp 0 is
        // deterministic and an identical retry would reproduce the same garbage.
        let mut parts: Option<Vec<Vec<usize>>> = None;
        for temperature in [0.0_f32, 0.2_f32] {
            match self.ask_model(prompt.clone(), temperature).await {
                Ok(answer_text) => {
                    parts = read_group_answer(&answer_text, titles.len());
                    if parts.is_some() {
                        break;
                    }
                }
                Err(_) => continue,
            }
        }

        let Some(parts) = parts else {
            eprintln!(
                "Story check could not read the AI answer for '{}': keeping the group as is",
                group.group_name
            );
            return vec![group];
        };

        if parts.len() > 1 {
            eprintln!(
                "Story check split '{}' into {} stories",
                group.group_name,
                parts.len()
            );
        }
        split_group_by_answer(&group, &parts)
    }

    async fn ask_model(&self, prompt: String, temperature: f32) -> anyhow::Result<String> {
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: Some(MessageContent::Text(prompt)),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }];

        let response = self
            .omlx
            .chat_completion(
                STORY_CHECK_MODEL,
                messages,
                temperature,
                CHECK_MAX_TOKENS,
                Some("low"),
            )
            .await?;

        Ok(response
            .choices
            .first()
            .and_then(|choice| choice.message.text_content())
            .unwrap_or_default())
    }
}
