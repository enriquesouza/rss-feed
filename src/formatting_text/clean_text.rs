use chrono::DateTime;

pub fn clean_title(title: &str) -> String {
    title
        .chars()
        .map(|character| {
            if character.is_alphanumeric() || character.is_whitespace() {
                character.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

// RSS/Atom costumam vir em dois formatos comuns:
// - RFC 3339: formato ISO 8601 com `T` e offset explícito, ex. `2026-04-14T09:30:00Z`
//   ou `2026-04-14T09:30:00-03:00`
// - RFC 2822: formato textual de e-mail/HTTP com dia da semana e mês abreviado,
//   ex. `Tue, 14 Apr 2026 09:30:00 -0300`
// Tentamos primeiro RFC 3339 e, se falhar, fazemos fallback para RFC 2822.
pub fn parse_feed_date(value: &str) -> Option<DateTime<chrono::FixedOffset>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .or_else(|| DateTime::parse_from_rfc2822(value).ok())
}

pub fn get_source_name(feed_url: &str) -> String {
    reqwest::Url::parse(feed_url)
        .ok()
        .and_then(|url| url.host_str().map(str::to_string))
        .unwrap_or_else(|| feed_url.to_string())
}
