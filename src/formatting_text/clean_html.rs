use html2text::from_read;

pub fn clean_html_text(input: &str) -> String {
    let safe_html = ammonia::clean(input);

    let text = match from_read(safe_html.as_bytes(), 5000) {
        Ok(text) => text,
        Err(_) => safe_html,
    };

    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
