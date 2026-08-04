// Real misclassified news from the live runs. Each row is a bug that used to
// happen because the first group in the YAML won, even on a fake match.

use rss_feed::app_data::rss_news::news_item::NewsItem;
use rss_feed::grouping_news::find_group_name::find_group_name;

fn make_news(title: &str, source: &str, link: &str, body: &str) -> NewsItem {
    NewsItem {
        source: source.to_string(),
        title: title.to_string(),
        link: link.to_string(),
        description: body.to_string(),
        clean_description: body.to_string(),
        published_at: "2026-08-04".to_string(),
    }
}

#[test]
fn picks_the_right_group_for_real_news() {
    let local_llama = "https://www.reddit.com/r/LocalLLaMA/comments/1vevsv9/more_qwen38_sizes/";

    let cases = [
        (
            "This Week in Rust 662",
            "this-week-in-rust.org",
            "https://this-week-in-rust.org/blog/2026/07/29/this-week-in-rust-662/",
            "does not mean a rust llm stream is safe to retry, doc benchmarks, analysis-stats inference",
            "rust-rustsec",
        ),
        (
            "This Week in Rust 661",
            "this-week-in-rust.org",
            "https://this-week-in-rust.org/blog/2026/07/22/this-week-in-rust-661/",
            "a production-grade rust sdk for the claude api",
            "rust-rustsec",
        ),
        (
            "Building secure Uniswap v4 hooks",
            "blog.trailofbits.com",
            "https://blog.trailofbits.com/2026/07/30/building-secure-uniswap-v4-hooks/",
            "because trusting the wrong poolkey opens an untrusted path and breaks the guarantees",
            "smart-contract-security",
        ),
        (
            "NVIDIA AI Supercomputer Comes Online at Naval Postgraduate School",
            "blogs.nvidia.com",
            "https://blogs.nvidia.com/blog/naval-postgraduate-school/",
            "to commission an nvidia dgx gb300 system",
            "ai-models",
        ),
        (
            "More Qwen3.8 sizes coming",
            "www.reddit.com",
            local_llama,
            "",
            "ai-models",
        ),
        (
            "Only 3 days ago",
            "www.reddit.com",
            "https://www.reddit.com/r/LocalLLaMA/comments/1veqt03/only_3_days_ago/",
            "",
            "ai-models",
        ),
        (
            "Special architecture in AFM3-20B instruction following pruning",
            "www.reddit.com",
            "https://www.reddit.com/r/LocalLLaMA/comments/1vep111/special_architecture/",
            "",
            "ai-models",
        ),
        (
            "I ran my own benchmarks on it",
            "www.reddit.com",
            "https://www.reddit.com/r/LocalLLaMA/comments/1vem999/i_ran_my_own_benchmarks/",
            "",
            "ai-models",
        ),
        // Source-only match: no keyword substring anywhere in title, link or body.
        (
            "Weekly digest #12",
            "importai.substack.com",
            "https://importai.substack.com/p/weekly-12",
            "",
            "ai-models",
        ),
        (
            "Fed holds benchmark interest rate steady",
            "cointelegraph.com",
            "https://cointelegraph.com/news/fed-holds-rate-steady",
            "",
            "macro-geopolitics",
        ),
        (
            "Trust Wallet users hit by phishing campaign",
            "ambcrypto.com",
            "https://ambcrypto.com/trust-wallet-phishing/",
            "victims followed what looked like a trusted path",
            "hacks-exploits",
        ),
        (
            "Solana hackathon winners announced",
            "solanafloor.com",
            "https://solanafloor.com/news/hackathon-winners",
            "",
            "solana-infra",
        ),
        (
            "SEC drops case against Ripple",
            "coindesk.com",
            "https://coindesk.com/policy/sec-drops-ripple",
            "",
            "regulation-market-structure",
        ),
        (
            "Bitcoin drifts sideways as traders wait",
            "ambcrypto.com",
            "https://ambcrypto.com/bitcoin-drifts-sideways/",
            "market optimism is based on flows",
            "general-market",
        ),
    ];

    for (title, source, link, body, expected) in cases {
        let item = make_news(title, source, link, body);
        assert_eq!(find_group_name(&item), expected, "title {title:?}");
    }
}

// News that LOOKS like AI but is not. None of these rows may land in ai-models.
#[test]
fn never_joins_ai_models_on_weak_or_slug_hints() {
    let cases = [
        // Two weak words (benchmark + inference) score 6, below the join bar of 8.
        (
            "PostgreSQL 19 benchmark: inference workloads run faster",
            "www.dbnews.com",
            "https://www.dbnews.com/postgresql-19-benchmark/",
            "",
        ),
        // Gemini is also a crypto exchange, so the name alone cannot decide.
        (
            "Gemini exchange wins New York trust licence",
            "www.coindesk.com",
            "https://www.coindesk.com/policy/gemini-trust-licence/",
            "",
        ),
        // blogs.nvidia.com is a general feed, so the source alone proves nothing.
        (
            "NVIDIA RTX 6090 brings ray tracing to indie games",
            "blogs.nvidia.com",
            "https://blogs.nvidia.com/blog/rtx-6090-indie-games/",
            "",
        ),
        // The keyword lives only in the link slug, and links never feed keywords.
        (
            "Weekend token launch roundup",
            "decrypt.co",
            "https://decrypt.co/llama-token-launch",
            "",
        ),
    ];

    for (title, source, link, body) in cases {
        let item = make_news(title, source, link, body);
        assert_ne!(find_group_name(&item), "ai-models", "title {title:?}");
    }
}
