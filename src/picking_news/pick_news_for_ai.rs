use crate::app_data::rss_news::news_item::NewsItem;
use crate::formatting_text::clean_text::clean_title;
use crate::picking_news::check_news::{is_low_quality, is_tech_or_security};
use crate::picking_news::limit_news_per_source::max_items_per_source;
use crate::picking_news::score_news::score_news;
use std::collections::{BTreeMap, HashSet};

const TARGET_TECH_ITEMS: usize = 40;
const MAX_ITEMS: usize = 80;

pub fn pick_news_for_ai(news: &[NewsItem]) -> Vec<NewsItem> {
    let (mut tech_items, mut other_items): (Vec<&NewsItem>, Vec<&NewsItem>) =
        news.iter().partition(|&item| is_tech_or_security(item));

    tech_items.sort_by_cached_key(|&item| std::cmp::Reverse(score_news(item)));
    other_items.sort_by_cached_key(|&item| std::cmp::Reverse(score_news(item)));

    let mut picked_items: Vec<&NewsItem> = Vec::new();
    let mut source_counts: BTreeMap<&str, usize> = BTreeMap::new();
    let mut seen_titles: HashSet<String> = HashSet::new();
    let mut seen_links: HashSet<&str> = HashSet::new();

    add_best_items(
        &tech_items,
        &mut picked_items,
        &mut source_counts,
        &mut seen_titles,
        &mut seen_links,
        TARGET_TECH_ITEMS,
    );
    let open_slots = MAX_ITEMS.saturating_sub(picked_items.len());
    add_best_items(
        &other_items,
        &mut picked_items,
        &mut source_counts,
        &mut seen_titles,
        &mut seen_links,
        open_slots,
    );

    picked_items.sort_by_cached_key(|&item| std::cmp::Reverse(score_news(item)));
    picked_items.truncate(MAX_ITEMS);

    picked_items.into_iter().cloned().collect()
}

fn add_best_items<'a>(
    items: &[&'a NewsItem],
    picked_items: &mut Vec<&'a NewsItem>,
    source_counts: &mut BTreeMap<&'a str, usize>,
    seen_titles: &mut HashSet<String>,
    seen_links: &mut HashSet<&'a str>,
    limit: usize,
) {
    let mut added = 0usize;

    for &item in items {
        if added >= limit {
            break;
        }

        if is_low_quality(item) && !is_tech_or_security(item) {
            continue;
        }

        let normalized_title = clean_title(&item.title);
        if !seen_titles.insert(normalized_title) {
            continue;
        }

        if !item.link.is_empty() && !seen_links.insert(item.link.as_str()) {
            continue;
        }

        let source_count = source_counts.entry(item.source.as_str()).or_default();
        if *source_count >= max_items_per_source(item) {
            continue;
        }

        *source_count += 1;
        picked_items.push(item);
        added += 1;
    }
}
