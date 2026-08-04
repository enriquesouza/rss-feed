use crate::app_data::news_group::NewsGroup;

/// Turn one group into one group per answer part. Parts keep the original
/// group_name, topic_words and tags; items keep their original (score-desc) order.
pub fn split_group_by_answer(group: &NewsGroup, parts: &[Vec<usize>]) -> Vec<NewsGroup> {
    if parts.len() == 1 {
        return vec![group.clone()];
    }

    parts
        .iter()
        .map(|part| {
            // Ascending index order preserves the score-desc item order,
            // so items.first() stays the best headline for the part.
            let mut sorted_part = part.clone();
            sorted_part.sort_unstable();

            let items = sorted_part
                .iter()
                .map(|&index| group.items[index - 1].clone())
                .collect();

            NewsGroup {
                group_name: group.group_name.clone(),
                topic_words: group.topic_words.clone(),
                tags: group.tags.clone(),
                items,
            }
        })
        .collect()
}
