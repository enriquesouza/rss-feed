pub fn format_text_for_telegram(input: &str) -> String {
    let fixed_breaks = input
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .replace("<br/><br/>", "\n\n")
        .replace("<br /><br />", "\n\n")
        .replace("<br><br>", "\n\n")
        .replace("<br/>", "\n")
        .replace("<br />", "\n")
        .replace("<br>", "\n")
        .replace("\\n\\n", "\n\n")
        .replace("\\n", "\n");

    fixed_breaks
        .split("\n\n")
        .map(|paragraph| {
            paragraph
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|paragraph| !paragraph.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}
