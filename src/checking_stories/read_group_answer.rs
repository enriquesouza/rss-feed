//! Injection-safe parser for the story-check answer.
//!
//! Safety argument: this parser never executes anything; its entire output
//! space is "a partition of this group's own items". The worst any hostile
//! title can achieve — even if the model obeys an embedded
//! `GROUP: 1,2,3,4,5,6` — is a single all-in-one group, which is exactly the
//! pre-existing heuristic behavior. Nothing can be leaked, dropped, or moved
//! across groups: any answer that is not an exact partition is rejected and
//! the caller keeps the original group.

/// Read the model answer into 1-based index groups.
/// Returns None when the answer is not an exact partition of 1..=item_count.
pub fn read_group_answer(answer: &str, item_count: usize) -> Option<Vec<Vec<usize>>> {
    // Drop any <think>...</think> preamble: keep only what follows the LAST
    // closing tag, so trial GROUP lines inside the thinking never leak in.
    let answer_text = match answer.rfind("</think>") {
        Some(position) => &answer[position + "</think>".len()..],
        None => answer,
    };

    let mut groups: Vec<Vec<usize>> = Vec::new();

    for line in answer_text.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("GROUP:") else {
            continue; // not a GROUP line: prose, markdown, injected text — ignore
        };

        // Accept only digits, commas and spaces, with at least one digit.
        let only_safe_chars = rest
            .chars()
            .all(|c| c.is_ascii_digit() || c == ',' || c == ' ');
        let has_digit = rest.chars().any(|c| c.is_ascii_digit());
        if !only_safe_chars || !has_digit {
            continue; // malformed GROUP line — ignore it like prose
        }

        let mut numbers: Vec<usize> = Vec::new();
        for piece in rest.split(',') {
            let piece = piece.trim();
            if piece.is_empty() {
                continue;
            }
            numbers.push(piece.parse::<usize>().ok()?);
        }
        if !numbers.is_empty() {
            groups.push(numbers);
        }
    }

    // Validate: the groups must be an exact partition of 1..=item_count.
    if groups.is_empty() {
        return None;
    }

    let mut seen = vec![false; item_count];
    for group in &groups {
        for &number in group {
            if number < 1 || number > item_count {
                return None; // out of range
            }
            if seen[number - 1] {
                return None; // repeated within or across groups
            }
            seen[number - 1] = true;
        }
    }
    if seen.iter().any(|covered| !covered) {
        return None; // some item is missing
    }

    Some(groups)
}
