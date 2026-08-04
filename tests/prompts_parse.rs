// Force every LazyLock prompt/rules file to parse with the same parser the app
// uses (serde_norway). A bad YAML edit fails here instead of at 3 AM in the loop.

#[test]
fn news_rules_yaml_parses() {
    let rules = &rss_feed::app_data::settings::news_rules::NEWS_RULES;
    assert!(
        rules.rss_feeds.len() >= 50,
        "expected all feeds, got {}",
        rules.rss_feeds.len()
    );
    assert!(
        rules
            .rss_feeds
            .iter()
            .any(|f| f.contains("qwenlm.github.io")),
        "AI feeds missing from rss_providers"
    );
}

// A keyword with a space around it can never match a whole word, so it would
// be dead weight in the YAML. Same for a keyword written in upper case.
#[test]
fn group_keywords_are_clean() {
    let rules = &rss_feed::app_data::settings::news_rules::NEWS_RULES;

    for group in &rules.topic_groups {
        for word in group.keywords.iter().chain(group.weak_keywords.iter()) {
            assert_eq!(
                word,
                word.trim(),
                "group {} keyword {word:?}",
                group.group_name
            );
            assert_eq!(
                word,
                &word.to_lowercase(),
                "group {} keyword {word:?}",
                group.group_name
            );
        }

        // Sources are matched with plain contains(), so any stray space or
        // upper-case letter would silently kill the match.
        for feed in &group.sources {
            assert_eq!(
                feed,
                feed.trim(),
                "group {} source {feed:?}",
                group.group_name
            );
            assert_eq!(
                feed,
                &feed.to_lowercase(),
                "group {} source {feed:?}",
                group.group_name
            );
        }
    }
}

#[test]
fn x_posts_prompt_parses() {
    let prompt = &rss_feed::writing_x_posts::write_x_posts_with_ai::X_POSTS_PROMPT;
    // Touching the LazyLock is the test — a parse failure panics here.
    let _ = &**prompt;
}
