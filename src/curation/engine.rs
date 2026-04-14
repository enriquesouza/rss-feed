use crate::curation::priority::compare_news_priority;
use crate::curation::signals::{is_low_signal_item, is_technical_or_security};
use crate::curation::sources::source_cap;
use crate::formatters::text::normalize_title;
use crate::models::rss::channel_row::ChannelRow;
use std::collections::BTreeMap;

const TARGET_TECHNICAL_ITEMS: usize = 40;
const MAX_ITEMS_FOR_LLM: usize = 80;

pub fn curate_news_for_llm(news: &[ChannelRow]) -> Vec<ChannelRow> {
    let mut technical = news
        .iter()
        .filter(|item| is_technical_or_security(item))
        .cloned()
        .collect::<Vec<_>>();
    let mut general = news
        .iter()
        .filter(|item| !is_technical_or_security(item))
        .cloned()
        .collect::<Vec<_>>();

    technical.sort_by(compare_news_priority);
    general.sort_by(compare_news_priority);

    let mut selected = Vec::new();
    let mut per_source: BTreeMap<String, usize> = BTreeMap::new();
    let mut seen_titles: Vec<String> = Vec::new();
    let mut seen_links: Vec<String> = Vec::new();

    collect_ranked_news(
        &technical,
        &mut selected,
        &mut per_source,
        &mut seen_titles,
        &mut seen_links,
        TARGET_TECHNICAL_ITEMS,
    );
    let remaining_slots = MAX_ITEMS_FOR_LLM.saturating_sub(selected.len());
    collect_ranked_news(
        &general,
        &mut selected,
        &mut per_source,
        &mut seen_titles,
        &mut seen_links,
        remaining_slots,
    );

    selected.sort_by(compare_news_priority);
    selected.truncate(MAX_ITEMS_FOR_LLM);
    selected
}

pub fn collect_ranked_news(
    candidates: &[ChannelRow],
    selected: &mut Vec<ChannelRow>,
    per_source: &mut BTreeMap<String, usize>,
    seen_titles: &mut Vec<String>,
    seen_links: &mut Vec<String>,
    limit: usize,
) {
    let mut added = 0usize;

    for item in candidates {
        if added >= limit {
            break;
        }

        if is_low_signal_item(item) && !is_technical_or_security(item) {
            continue;
        }

        let normalized_title = normalize_title(&item.title);
        if seen_titles.iter().any(|title| title == &normalized_title) {
            continue;
        }

        if !item.link.is_empty() && seen_links.iter().any(|link| link == &item.link) {
            continue;
        }

        let source_count = per_source.entry(item.source.clone()).or_default();
        if *source_count >= source_cap(item) {
            continue;
        }

        *source_count += 1;
        seen_titles.push(normalized_title);
        if !item.link.is_empty() {
            seen_links.push(item.link.clone());
        }
        selected.push(item.clone());
        added += 1;
    }
}
