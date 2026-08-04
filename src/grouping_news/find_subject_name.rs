/// Plain subject name for the news browser: the buckets a reader thinks in
/// (IA, Rust, Hacking, Crypto), derived from the group name.
pub fn find_subject_name(group_name: &str) -> String {
    let subject = match group_name {
        "ai-models" | "ai-agents-security" => "IA",
        "rust-rustsec" => "Rust",
        "hacks-exploits" | "smart-contract-security" => "Hacking",
        "stablecoins-payments"
        | "btc-corporate"
        | "ethereum-evm"
        | "solana-infra"
        | "regulation-market-structure" => "Crypto",
        "macro-geopolitics" => "Macro",
        _ => "Geral",
    };
    subject.to_string()
}
