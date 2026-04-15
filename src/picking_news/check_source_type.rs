use crate::app_data::settings::app_env::NEWS_RULES;

pub fn is_high_volume_source(source: &str) -> bool {
    NEWS_RULES
        .high_volume_sources
        .iter()
        .any(|candidate| source.contains(candidate))
}
