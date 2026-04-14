use crate::curation::signals::is_technical_or_security;
use crate::models::configs::config::CURATION_CONFIG;
use crate::models::rss::channel_row::ChannelRow;

pub fn source_cap(item: &ChannelRow) -> usize {
    if is_technical_or_security(item) {
        4
    } else if is_high_volume_general_source(&item.source) {
        3
    } else {
        2
    }
}

pub fn is_high_volume_general_source(source: &str) -> bool {
    CURATION_CONFIG
        .high_volume_general_sources
        .iter()
        .any(|candidate| source.contains(candidate))
}
