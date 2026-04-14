use crate::clustering::signatures::is_generic_cluster_token;
use crate::formatters::text::normalize_title;
use crate::models::rss::channel_row::ChannelRow;
use std::collections::HashSet;

pub fn same_story(left: &ChannelRow, right: &ChannelRow) -> bool {
    normalize_title(&left.title) == normalize_title(&right.title)
        || (!left.link.is_empty() && left.link == right.link)
}

pub fn signature_overlap(left: &[String], right: &[String]) -> usize {
    let left_set: HashSet<_> = left.iter().collect();
    let right_set: HashSet<_> = right.iter().collect();
    left_set.intersection(&right_set).count()
}

pub fn has_specific_signature_overlap(left: &[String], right: &[String]) -> bool {
    left.iter().any(|token| {
        right.iter().any(|candidate| candidate == token) && !is_generic_cluster_token(token)
    })
}
