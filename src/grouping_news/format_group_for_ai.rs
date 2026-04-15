use crate::app_data::news_group::NewsGroup;
use crate::grouping_news::score_news_group::count_unique_sources;
const MAX_ARTICLES_PER_GROUP: usize = 6;

pub fn format_group_for_ai(group: &NewsGroup) -> String {
    let unique_sources = count_unique_sources(group);
    let repeat = match unique_sources {
        0 | 1 => "single_source",
        2 | 3 => "repeated",
        _ => "very_repeated",
    };

    let sample_titles = group
        .items
        .iter()
        .take(4)
        .map(|item| format!("- {}", item.title))
        .collect::<Vec<_>>()
        .join("\n");

    let articles = group
        .items
        .iter()
        .take(MAX_ARTICLES_PER_GROUP)
        .map(|item| {
            format!(
                "- [{}] {} | {}\n  {}",
                item.source, item.published_at, item.title, item.clean_description
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "[GROUP] {}\n[REPEAT] {}\n[COUNT] {}\n[SOURCES] {}\n[WORDS] {}\n[TAGS] {}\n[SAMPLES]\n{}\n[ARTICLES]\n{}",
        group.group_name,
        repeat,
        group.items.len(),
        unique_sources,
        group.topic_words.join(", "),
        group.tags.join(", "),
        sample_titles,
        articles
    )
}
