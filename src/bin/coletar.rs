//! One fetch round, on demand — the same path the pipeline takes, but it stops
//! after saving the consolidated stories so the portal can be filled without
//! waiting three hours (and without sending anything to Telegram).
//!
//!   cargo run --bin coletar            → fetch, group, save (no AI summaries)
//!   cargo run --bin coletar -- resumos → also write the PT digest of each story
//!
//! Grouping is NOT by subject: it is the real `group_related_news`, which only
//! joins items that share topic words — the same fact told by several feeds.

use rss_feed::app_data::news_group::NewsGroup;
use rss_feed::checking_stories::check_story_with_ai::StoryChecker;
use rss_feed::fetching_rss::fetch_rss_news::fetch_rss_news;
use rss_feed::grouping_news::find_subject_name::find_subject_name;
use rss_feed::grouping_news::format_group_for_ai::format_group_for_ai;
use rss_feed::grouping_news::group_related_news::group_related_news;
use rss_feed::grouping_news::score_news_group::score_news_group;
use rss_feed::picking_news::pick_news_for_ai::pick_news_for_ai;
use rss_feed::storing_posts::save_and_update_posts::{NewsItemRow, PostsDb};
use rss_feed::writing_news::write_news_with_ai::NewsWriter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    dotenvy::from_path(&env_file)
        .or_else(|_| dotenvy::dotenv().map(|_| ()))
        .ok();

    let with_summaries = std::env::args().any(|a| a == "resumos");

    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .connect_timeout(std::time::Duration::from_secs(10))
        .user_agent("rss-feed/0.1")
        .build()?;
    let posts_db = PostsDb::open_posts_db()?;

    println!("Buscando os feeds...");
    let news = fetch_rss_news(&client)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    println!("  {} notícias baixadas", news.len());

    let picked = pick_news_for_ai(&news);
    println!("  {} escolhidas para agrupar", picked.len());

    let groups: Vec<NewsGroup> = group_related_news(&picked);
    // Second layer: ask the AI to un-mix groups the heuristics over-joined.
    let checker = StoryChecker::new(&client);
    let groups = checker.check_stories(groups).await;
    println!("  {} histórias formadas\n", groups.len());

    let run_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let writer = NewsWriter::new(&client);

    for group in &groups {
        let score = score_news_group(group) as i64;
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

        let digest = if with_summaries {
            match writer.write_news_message(format_group_for_ai(group)).await {
                Ok(Some(text)) => Some(text.replace("(Muito repetida)", "").trim().to_string()),
                Ok(None) => None,
                Err(error) => {
                    eprintln!("  resumo falhou para '{}': {error}", group.group_name);
                    None
                }
            }
        } else {
            None
        };

        posts_db.save_news_story(
            &run_at,
            &subject,
            &headline,
            score,
            digest.as_deref(),
            &items,
        )?;

        println!(
            "[{subject}] peso {score} · {} notícias · tema \"{}\"",
            group.items.len(),
            group.group_name
        );
        for item in &group.items {
            println!("    - {} ({})", item.title, item.source);
        }
    }

    println!("\nPronto. Abra http://localhost:8787 → notícias");
    Ok(())
}
