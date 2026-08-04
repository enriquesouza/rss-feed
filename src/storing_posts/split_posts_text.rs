use crate::storing_posts::save_and_update_posts::NewPost;

/// Split the AI writer output into structured posts.
///
/// Expected blocks (tolerant to small format drift):
///   POST 1 — topic
///   TEXTO: ...
///   QUOTE: ...
///   AIZZY: ...
///
/// If nothing parses, the whole text becomes one post so nothing is lost.
pub fn split_posts_text(posts_text: &str) -> Vec<NewPost> {
    let posts_text = &strip_think_blocks(posts_text);
    let mut posts: Vec<NewPost> = Vec::new();
    let mut current: Option<NewPost> = None;
    let mut last_field: Option<&'static str> = None;

    for raw_line in posts_text.lines() {
        let line = raw_line.trim();

        if is_post_header(line) {
            if let Some(post) = current.take() {
                posts.push(post);
            }
            current = Some(NewPost {
                topic: header_topic(line),
                text_en: String::new(),
                quote_url: None,
                aizzy_text: None,
            });
            last_field = None;
            continue;
        }

        let Some(post) = current.as_mut() else {
            continue;
        };

        if let Some(rest) = field_value(line, "TEXTO") {
            post.text_en = strip_model_notes(rest);
            last_field = Some("texto");
        } else if let Some(rest) = field_value(line, "QUOTE") {
            let clean = rest.trim();
            if !clean.is_empty() && clean.to_lowercase() != "postar direto" {
                post.quote_url = Some(clean.to_string());
            }
            last_field = Some("quote");
        } else if let Some(rest) = field_value(line, "AIZZY") {
            let clean = rest.trim();
            if !clean.is_empty() && !clean.to_lowercase().contains("não se aplica") {
                post.aizzy_text = Some(clean.to_string());
            }
            last_field = Some("aizzy");
        } else if !line.is_empty() {
            // Continuation line of the previous field (multi-line TEXTO/AIZZY).
            match last_field {
                Some("texto") => {
                    if !post.text_en.is_empty() {
                        post.text_en.push(' ');
                    }
                    post.text_en.push_str(line);
                }
                Some("aizzy") => {
                    if let Some(aizzy) = post.aizzy_text.as_mut() {
                        aizzy.push(' ');
                        aizzy.push_str(line);
                    }
                }
                _ => {}
            }
        }
    }

    if let Some(post) = current.take() {
        posts.push(post);
    }

    // Drop template echoes (the model repeating the prompt's format line,
    // e.g. "<max 260 chars, one clear idea...>") and empty posts.
    posts.retain(|p| {
        let text = p.text_en.trim();
        !(text.is_empty() || (text.starts_with('<') && text.ends_with('>')))
    });

    // Fallback keeps unparsed output — but only post-sized text. A wall of
    // instructions echoed by the model is garbage, not a lost post.
    if posts.is_empty() && !posts_text.trim().is_empty() && posts_text.trim().len() <= 600 {
        posts.push(NewPost {
            topic: "dia".to_string(),
            text_en: posts_text.trim().to_string(),
            quote_url: None,
            aizzy_text: None,
        });
    }

    posts
}

/// Cut the model's own commentary off the end of a post.
/// Small local models like to append notes ("— Check length: ~248 chars.",
/// "Engineer POV: ...") after the text. Those must never reach X.
fn strip_model_notes(text: &str) -> String {
    const NOTE_MARKERS: [&str; 9] = [
        "— Check",
        "- Check",
        "Check length",
        "Check chars",
        "— Engineer POV",
        "- Engineer POV",
        "Engineer POV",
        "(Note:",
        "Note to self",
    ];
    let mut clean = text.to_string();
    for marker in NOTE_MARKERS {
        if let Some(cut) = clean.find(marker) {
            clean.truncate(cut);
        }
    }
    clean
        .trim()
        .trim_end_matches(['—', '-', '·'])
        .trim()
        .to_string()
}

/// Remove <think>...</think> reasoning blocks some models leak into the output.
/// An unclosed <think> means everything after it is reasoning — keep only what
/// came before, unless nothing did.
fn strip_think_blocks(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    loop {
        match rest.find("<think>") {
            Some(open) => {
                out.push_str(&rest[..open]);
                match rest[open..].find("</think>") {
                    Some(close) => rest = &rest[open + close + "</think>".len()..],
                    None => break,
                }
            }
            None => {
                out.push_str(rest);
                break;
            }
        }
    }
    if out.trim().is_empty() {
        // Whole answer was inside think tags — fall back to the raw text.
        text.to_string()
    } else {
        out
    }
}

fn is_post_header(line: &str) -> bool {
    let upper = line.to_uppercase();
    upper.starts_with("POST ") || upper.starts_with("**POST ")
}

fn header_topic(line: &str) -> String {
    // "POST 1 — some topic" -> "some topic"
    for sep in ["—", "–", "-", ":"] {
        if let Some(pos) = line.find(sep) {
            let topic = line[pos + sep.len()..]
                .trim()
                .trim_matches(['*', '[', ']'])
                .trim();
            if !topic.is_empty() {
                return topic.to_string();
            }
        }
    }
    "sem tema".to_string()
}

fn field_value<'a>(line: &'a str, field: &str) -> Option<&'a str> {
    let clean = line.trim_start_matches(['*', '-', ' ']);
    let upper = clean.to_uppercase();
    let prefix = format!("{field}:");
    if upper.starts_with(&prefix) {
        Some(clean[prefix.len()..].trim())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::split_posts_text;

    #[test]
    fn splits_three_posts() {
        let text = "POST 1 — Qwen release\nTEXTO: Qwen shipped a 32B model.\nQUOTE: https://x.com/a/1\nAIZZY: Route it on aizzy.ai\n\nPOST 2 — Ollama\nTEXTO: Ollama got faster.\nQUOTE: postar direto\nAIZZY: não se aplica\n\nPOST 3 — Rust\nTEXTO: Rust 2.0 is out.\nQUOTE: https://x.com/b/2\nAIZZY: não se aplica";
        let posts = split_posts_text(text);
        assert_eq!(posts.len(), 3);
        assert_eq!(posts[0].topic, "Qwen release");
        assert_eq!(posts[0].quote_url.as_deref(), Some("https://x.com/a/1"));
        assert_eq!(posts[0].aizzy_text.as_deref(), Some("Route it on aizzy.ai"));
        assert!(posts[1].quote_url.is_none());
        assert!(posts[1].aizzy_text.is_none());
    }

    #[test]
    fn falls_back_to_one_post() {
        let posts = split_posts_text("just some text without any structure");
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].topic, "dia");
    }

    #[test]
    fn cuts_model_notes_off_the_post() {
        let text = "POST 1 — LFM2\nTEXTO: LFM2 enables fast CPU inference. How many tokens does your pipeline handle? — Check length: ~248 chars. Good. — Engineer POV: Focuses on CPU.\nQUOTE: postar direto\nAIZZY: não se aplica";
        let posts = split_posts_text(text);
        assert_eq!(posts.len(), 1);
        assert_eq!(
            posts[0].text_en,
            "LFM2 enables fast CPU inference. How many tokens does your pipeline handle?"
        );
    }

    #[test]
    fn drops_think_blocks_and_template_echo() {
        let text = "<think>POST 1 — draft\nTEXTO: counting chars...</think>\nPOST 1 — [short topic]\nTEXTO: <max 260 chars, one clear idea>\n\nPOST 2 — Real topic\nTEXTO: Qwen ships real weights today.\nQUOTE: postar direto\nAIZZY: não se aplica";
        let posts = split_posts_text(text);
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].topic, "Real topic");
        assert_eq!(posts[0].text_en, "Qwen ships real weights today.");
    }

    #[test]
    fn cuts_check_chars_note() {
        let text = "POST 1 — Agentes\nTEXTO: How do you scope API keys for autonomous agents? Check chars: ~248. Good.\nQUOTE: postar direto\nAIZZY: não se aplica";
        let posts = split_posts_text(text);
        assert_eq!(
            posts[0].text_en,
            "How do you scope API keys for autonomous agents?"
        );
    }

    #[test]
    fn drops_giant_prompt_echo_in_fallback() {
        let echo = "x".repeat(1200);
        let posts = split_posts_text(&echo);
        assert!(posts.is_empty());
    }
}
