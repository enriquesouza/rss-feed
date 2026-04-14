use html2text::from_read;

pub fn sanitize_rss_text(input: &str) -> String {
    let clean_html = ammonia::clean(input);

    let text = match from_read(clean_html.as_bytes(), 5000) {
        Ok(text) => text,
        Err(_) => clean_html,
    };

    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
