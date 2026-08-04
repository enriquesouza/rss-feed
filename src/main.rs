use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use dotenvy::dotenv;
use std::collections::HashSet;
use std::error::Error;
use std::time::Duration as StdDuration;
use tokio::time::{Duration, sleep};

pub mod app_data;
pub mod checking_stories;
pub mod fetching_rss;
pub mod fetching_x_metrics;
pub mod formatting_text;
pub mod grouping_news;
pub mod making_posts_today;
pub mod picking_news;
pub mod reading_news_today;
pub mod reading_stories_today;
pub mod sending_to_telegram;
pub mod storing_posts;
pub mod writing_news;
pub mod writing_x_posts;

use crate::app_data::news_group::NewsGroup;
use crate::app_data::rss_news::news_item::NewsItem;
use crate::app_data::settings::news_rules::NEWS_RULES;
use crate::app_data::telegram::telegram_response::TelegramResponse;
use crate::checking_stories::check_story_with_ai::StoryChecker;
use crate::fetching_rss::fetch_rss_news::fetch_rss_news;
use crate::grouping_news::find_subject_name::find_subject_name;
use crate::grouping_news::format_group_for_ai::format_group_for_ai;
use crate::grouping_news::group_related_news::{group_related_news, make_all_news_groups};
use crate::grouping_news::score_news_group::score_news_group;
use crate::making_posts_today::save_and_check_posts_made_today::PostsMadeTodayDb;
use crate::picking_news::pick_news_for_ai::pick_news_for_ai;
use crate::reading_news_today::save_and_check_news_read_today::NewsReadTodayDb;
use crate::reading_stories_today::save_and_check_stories_read_today::StoriesReadTodayDb;
use crate::sending_to_telegram::send_to_telegram::send_to_telegram;
use crate::storing_posts::save_and_update_posts::{NewsItemRow, PostsDb};
use crate::storing_posts::split_posts_text::split_posts_text;
use crate::writing_news::write_news_with_ai::NewsWriter;
use crate::writing_x_posts::write_x_posts_with_ai::XPostsWriter;

use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    crate::app_data::settings::app_env::AppEnv::check_telegram_is_set()?;

    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .connect_timeout(StdDuration::from_secs(10))
        .pool_idle_timeout(StdDuration::from_secs(90))
        .pool_max_idle_per_host(4)
        .redirect(reqwest::redirect::Policy::limited(5))
        .tcp_keepalive(StdDuration::from_secs(30))
        .user_agent("rss-feed/0.1")
        .build()?;
    let writer: NewsWriter = NewsWriter::new(&client);
    let x_posts_writer: XPostsWriter = XPostsWriter::new(&client);
    let story_checker: StoryChecker = StoryChecker::new(&client);
    let news_read_today_db: NewsReadTodayDb = NewsReadTodayDb::open_news_read_today_db()?;
    let stories_read_today_db: StoriesReadTodayDb =
        StoriesReadTodayDb::open_stories_read_today_db()?;
    let posts_made_today_db: PostsMadeTodayDb = PostsMadeTodayDb::open_posts_made_today_db()?;
    let posts_db: PostsDb = PostsDb::open_posts_db()?;

    loop {
        let news_result: Result<Vec<NewsItem>, Box<dyn Error>> = fetch_rss_news(&client).await;

        match news_result {
            Ok(news_list) if news_list.is_empty() => {
                // Falls through to the shared sleep block at bottom of the loop — no redundant message.
            }
            Ok(news_list) => {
                let fresh_news =
                    news_read_today_db.filter_out_news_already_read_today(news_list)?;
                if fresh_news.is_empty() {
                    eprintln!("No fresh news after dedup, sleeping 3h...");
                    sleep(Duration::from_hours(3)).await;
                    continue;
                }

                news_read_today_db.save_news_read_today(&fresh_news)?;

                let story_groups = make_all_news_groups(&fresh_news);
                let fresh_story_groups = stories_read_today_db
                    .filter_out_stories_already_read_today(&client, story_groups)
                    .await?;
                if fresh_story_groups.is_empty() {
                    eprintln!("No fresh story groups after dedup, sleeping 3h...");
                    sleep(Duration::from_hours(3)).await;
                    continue;
                }

                stories_read_today_db.save_stories_read_today(&fresh_story_groups)?;

                let fresh_story_news = fresh_story_groups
                    .into_iter()
                    .flat_map(|group| group.items)
                    .collect::<Vec<_>>();

                let picked_news = pick_news_for_ai(&fresh_story_news);
                if picked_news.is_empty() {
                    eprintln!("No news picked for AI, sleeping 3h...");
                    sleep(Duration::from_hours(3)).await;
                    continue;
                }

                let news_groups: Vec<_> = group_related_news(&picked_news);
                // Second layer: ask the AI to un-mix groups the heuristics over-joined.
                let news_groups = story_checker.check_stories(news_groups).await;

                if news_groups.is_empty() {
                    eprintln!("No news groups after grouping, sleeping 3h...");
                    sleep(Duration::from_hours(3)).await;
                    continue;
                }

                // ---- Score every group up-front so ordering is deterministic by priority. ----
                let scored_groups: Vec<(i32, NewsGroup)> = news_groups
                    .into_iter()
                    .map(|group| (score_news_group(&group), group))
                    .collect();

                struct GroupMeta {
                    topic_label: String,
                    source_domains: Vec<String>,
                }

                // Tuple: (score, meta, PT digest text, ENGLISH source text).
                // The English source feeds the X posts writer so posts stay
                // English end-to-end (the PT digest is only for Telegram).
                let mut processed_entries: Vec<(i32, GroupMeta, String, String, NewsGroup)> =
                    stream::iter(scored_groups.into_iter())
                        .map(|(score, group)| {
                            let writer_ref = &writer;
                            async move {
                                let topic_label = topic_label_for(&group.group_name);
                                let source_domains = extract_source_domains(&group.items);
                                let meta = GroupMeta {
                                    topic_label,
                                    source_domains,
                                };
                                let group_text = format_group_for_ai(&group);
                                let english_source = group_text.clone();
                                match writer_ref.write_news_message(group_text).await {
                                    Ok(Some(msg)) => {
                                        let clean_msg =
                                            msg.replace("(Muito repetida)", "").trim().to_string();
                                        Some((score, meta, clean_msg, english_source, group))
                                    }
                                    Ok(None) => None,
                                    Err(e) => {
                                        eprintln!(
                                            "AI writer failed for group {}: {}",
                                            group.group_name, e
                                        );
                                        None
                                    }
                                }
                            }
                        })
                        .buffer_unordered(5)
                        .filter_map(|res| async move { res })
                        .collect()
                        .await;

                if processed_entries.is_empty() {
                    eprintln!("All AI writer calls failed or returned empty, sleeping 3h...");
                    sleep(Duration::from_hours(3)).await;
                    continue;
                }

                // Sort entries by priority score desc so the reader always sees the most important
                // categories first — regardless of which async task finished last.
                processed_entries.sort_by_key(|e| std::cmp::Reverse(e.0));

                // ---- Save this run's consolidated stories: the AI summary, the
                // importance score the pipeline computed, and the items behind it. ----
                let run_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                for (score, _meta, digest_pt, _english, group) in processed_entries.iter() {
                    let subject = find_subject_name(&group.group_name);
                    let headline = group
                        .items
                        .first()
                        .map(|item| item.title.clone())
                        .unwrap_or_else(|| group.group_name.clone());
                    let items: Vec<NewsItemRow> = group
                        .items
                        .iter()
                        .map(|item| NewsItemRow {
                            run_at: run_at.clone(),
                            topic: subject.clone(),
                            title: item.title.clone(),
                            link: Some(item.link.clone()),
                            source: Some(item.source.clone()),
                        })
                        .collect();
                    if let Err(error) = posts_db.save_news_story(
                        &run_at,
                        &subject,
                        &headline,
                        *score as i64,
                        Some(digest_pt.as_str()),
                        &items,
                    ) {
                        eprintln!("Could not save news story to sqlite: {error}");
                    }
                }

                // Keep the top stories to make the X post drafts later (once per day).
                // Uses the ENGLISH source text (entry.3), never the PT digest.
                let top_stories_for_posts: Vec<String> = processed_entries
                    .iter()
                    .take(8)
                    .map(|entry| entry.3.clone())
                    .collect();

                // Partition: non-empty topic-label entries first (still desc by score),
                // then general-market / unknown-topic fallbacks at the very end.
                // ---- Partition entries by topic label presence (still in desc-score order). ----
                let total_count = processed_entries.len();

                let mut non_empty = Vec::new();
                let mut empty_topic = Vec::new();
                for entry in processed_entries.into_iter() {
                    if entry.1.topic_label.is_empty() {
                        empty_topic.push(entry);
                    } else {
                        non_empty.push(entry);
                    }
                }

                // ---- Build the final Telegram message. ----
                let unique_topics: HashSet<String> =
                    non_empty.iter().map(|e| e.1.topic_label.clone()).collect();

                let mut final_message = String::new();
                final_message.push_str(&format!(
                    "🗞 *Resumo de hoje* — {} notícias em {} categorias\n",
                    total_count,
                    unique_topics.len(),
                ));
                final_message.push('\n');

                for (_score, meta, ai_text, _english_source, _group) in
                    non_empty.into_iter().chain(empty_topic.into_iter())
                {
                    if !meta.topic_label.is_empty() {
                        final_message.push_str(&format!("\n{}\n", meta.topic_label));
                    } else {
                        final_message.push('\n');
                    }
                    final_message.push('\n');
                    final_message.push_str(&ai_text);

                    if !meta.source_domains.is_empty() {
                        let links: Vec<&str> =
                            meta.source_domains.iter().map(|s| s.as_str()).collect();
                        final_message.push_str(&format!("\n\n🔗 {}", links.join(" · ")));
                    }
                    final_message.push('\n');
                }

                // ---- Telegram split (unchanged from original). ----
                let mut parts = Vec::new();
                let mut start = 0;
                while start < final_message.len() {
                    let mut end = (start + 4096).min(final_message.len());
                    while end > start && !final_message.is_char_boundary(end) {
                        end -= 1;
                    }
                    parts.push(final_message[start..end].to_string());
                    start = end;
                }

                for part in parts {
                    let send_result: Result<TelegramResponse, Box<dyn Error>> =
                        send_to_telegram(&client, part).await;
                    if let Err(error) = send_result {
                        eprintln!("Could not send news to Telegram: {error}");
                        break;
                    }
                }

                // ---- Make the X post drafts, once per day, from today's top stories. ----
                make_x_posts_once_per_day(
                    &client,
                    &x_posts_writer,
                    &posts_made_today_db,
                    &posts_db,
                    &top_stories_for_posts,
                )
                .await;
            }
            Err(e) => {
                eprintln!("RSS fetch failed: {}", e);
            }
        }

        eprintln!("Sleeping 3 hours before next fetch...");
        sleep(Duration::from_hours(3)).await;
    }
}

/// Return a display-friendly, emoji-prefixed topic label for the given group name.
/// Falls back to an empty string when no mapping is found (general-market / unknown).
fn topic_label_for(group_name: &str) -> String {
    let raw = match NEWS_RULES
        .topic_groups
        .iter()
        .find(|r| r.group_name == group_name)
    {
        Some(r) => match r.group_name.as_str() {
            "ai-models" => Some("🤖 *AI Models*"),
            "rust-rustsec" => Some("⚙️ *Rust/Solidity*"),
            "smart-contract-security" => Some("🛡️ *Smart Contract Security*"),
            "hacks-exploits" => Some("🔴 *Hacks & Exploits*"),
            "stablecoins-payments" => Some("💵 *Stablecoins & Payments*"),
            "regulation-market-structure" => Some("📜 *Regulation*"),
            "btc-corporate" => Some("₿ *Corporate Treasury*"),
            "ethereum-evm" => Some("Ξ *Ethereum & EVM*"),
            "solana-infra" => Some("◎ *Solana Infra*"),
            "macro-geopolitics" => Some("🌐 *Macro & Geopolitics*"),
            "ai-agents-security" => Some("🤖 *AI Agents & Security*"),
            _ => None, // general-market or any unrecognized group → empty string
        },
        None => return String::new(),
    };
    raw.map(String::from).unwrap_or_default()
}

/// Extract unique source hostnames (in insertion order) from a slice of news items.
/// No external crates — simple scheme/hostname parsing against `item.source`.
fn extract_source_domains(items: &[NewsItem]) -> Vec<String> {
    let mut seen = HashSet::<String>::new();
    let mut domains = Vec::new();

    for item in items {
        // Strip `scheme://` if present, then take up to the next `/` or whole string.
        let host_part: &str = match (item.source.find("://"), item.source.find('/')) {
            (Some(scheme_end), Some(next_slash)) if scheme_end + 3 < next_slash => {
                &item.source[scheme_end + 3..]
            }
            _ => item.source.as_str(),
        };

        // Take the hostname portion.
        let hostname: &str = match host_part.find('/') {
            Some(idx) => &host_part[..idx],
            None => host_part,
        };

        // Strip a trailing port segment if present (e.g. `:443`).
        let hostname: String = match hostname.rfind(':') {
            Some(port_start)
                if hostname[port_start + 1..]
                    .chars()
                    .all(|c| c.is_ascii_digit()) =>
            {
                hostname[..port_start].to_string()
            }
            _ => hostname.to_string(),
        };

        let host = hostname.trim();
        if !host.is_empty() && seen.insert(host.to_string()) {
            domains.push(host.to_string());
        }
    }

    domains
}

/// Make the day's X post drafts from the top stories and send them to Telegram
/// for human review. Runs at most once per day; the human always posts by hand.
async fn make_x_posts_once_per_day(
    client: &reqwest::Client,
    x_posts_writer: &XPostsWriter<'_>,
    posts_made_today_db: &PostsMadeTodayDb,
    posts_db: &PostsDb,
    top_stories: &[String],
) {
    if top_stories.is_empty() {
        return;
    }

    match posts_made_today_db.check_posts_made_today() {
        Ok(true) => return,
        Ok(false) => {}
        Err(error) => {
            eprintln!("Could not check posts-made db: {error}");
            return;
        }
    }

    let day_stories_text = top_stories.join("\n\n---\n\n");

    let posts_text = match x_posts_writer.write_x_posts(day_stories_text).await {
        Ok(Some(text)) => text,
        Ok(None) => {
            eprintln!("X posts writer returned empty, will retry next loop");
            return;
        }
        Err(error) => {
            eprintln!("X posts writer failed, will retry next loop: {error}");
            return;
        }
    };

    // Save every draft to SQLite first — the admin page manages them from here.
    let new_posts = split_posts_text(&posts_text);
    match posts_db.save_new_posts(&new_posts) {
        Ok(ids) => eprintln!("Saved {} post drafts to sqlite: {:?}", ids.len(), ids),
        Err(error) => {
            // Nothing was saved, so there is nothing to announce — retry next loop.
            eprintln!("Could not save post drafts to sqlite, will retry next loop: {error}");
            return;
        }
    }

    // The drafts ARE the deliverable — mark the day done as soon as they are
    // saved. If Telegram fails below, the drafts still exist in the admin;
    // marking late used to duplicate the whole batch on the next 3h cycle.
    if let Err(error) = posts_made_today_db.save_posts_made_today() {
        eprintln!("Could not save posts-made flag: {error}");
    }

    let full_message = format!(
        "📝 *Posts de hoje — revise e publique*\n_Gerencie no admin: http://localhost:8787_\n\n{}",
        posts_text
    );

    // Same 4096-char split rule the news message uses.
    let mut parts: Vec<String> = Vec::new();
    let mut start = 0;
    while start < full_message.len() {
        let mut end = (start + 4096).min(full_message.len());
        while end > start && !full_message.is_char_boundary(end) {
            end -= 1;
        }
        parts.push(full_message[start..end].to_string());
        start = end;
    }

    for part in parts {
        if let Err(error) = send_to_telegram(client, part).await {
            // Notification only — the drafts are already saved and the day is
            // already marked, so a failed send never duplicates the batch.
            eprintln!("Could not send X posts to Telegram: {error}");
            return;
        }
    }
}
