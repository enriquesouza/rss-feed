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

    let mut result = String::with_capacity(fixed_breaks.len());
    let mut first_paragraph = true;

    for paragraph in fixed_breaks.split("\n\n") {
        let mut p_result = String::new();
        let mut first_line = true;
        for line in paragraph.lines().map(str::trim).filter(|l| !l.is_empty()) {
            if !first_line {
                p_result.push(' ');
            }
            p_result.push_str(line);
            first_line = false;
        }

        if !p_result.is_empty() {
            if !first_paragraph {
                result.push_str("\n\n");
            }
            result.push_str(&p_result);
            first_paragraph = false;
        }
    }

    result
}
