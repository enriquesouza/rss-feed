// Looks for a keyword as a real word, not as a piece of another word.
// "rust" is not inside "trusting", but "qwen" is inside "qwen3.8-27b".
pub fn check_if_word_is_there(text: &str, word: &str) -> bool {
    if word.is_empty() {
        return false;
    }

    text.match_indices(word)
        .any(|(start, _)| starts_a_word(text, start) && ends_a_word(text, start + word.len()))
}

// Nothing letter-or-number may come right before the keyword.
fn starts_a_word(text: &str, start: usize) -> bool {
    match text[..start].chars().next_back() {
        None => true,
        Some(letter) => !letter.is_alphanumeric(),
    }
}

// After the keyword only a normal plural/verb ending may come.
// A digit is fine too, so "gpt5" and "llama4" still match.
fn ends_a_word(text: &str, end: usize) -> bool {
    // Slice out the run of letters right after the keyword — no String needed.
    let after = &text[end..];
    let stop = after
        .char_indices()
        .find(|(_, letter)| !letter.is_alphabetic())
        .map(|(index, _)| index)
        .unwrap_or(after.len());

    matches!(
        &after[..stop],
        "" | "s" | "es" | "ed" | "ing" | "er" | "ers"
    )
}

#[cfg(test)]
mod tests {
    use super::check_if_word_is_there;

    #[test]
    fn finds_only_real_words() {
        let cases = [
            // Bugs seen in production: the keyword hid inside another word.
            ("trusting the wrong poolkey", "rust", false),
            ("blog.rust-lang.org", "rust", true),
            ("guarantees", "tee", false),
            ("a tee attestation", "tee", true),
            ("solodit database", "base", false),
            ("based on the report", "base", false),
            ("defillama.com", "llama", false),
            ("llama.cpp speedup", "llama", true),
            // Model names glued to digits and dots must keep matching.
            ("qwen3.8-27b released", "qwen", true),
            ("gpt5 is out", "gpt", true),
            ("rustsec advisory", "sec", false),
            ("secure hooks", "sec", false),
            ("sec's ruling", "sec", true),
            ("solana hackathon", "hack", false),
            ("wallet hacked", "hack", true),
            ("stablecoins grow", "stablecoin", true),
            ("leipzig meetup", "eip", false),
            ("a smart contract bug", "smart contract", true),
        ];

        for (text, word, expected) in cases {
            assert_eq!(
                check_if_word_is_there(text, word),
                expected,
                "text {text:?} word {word:?}"
            );
        }
    }

    #[test]
    fn empty_word_never_matches() {
        assert!(!check_if_word_is_there("anything", ""));
    }
}
