# rss-feed

This file explains why each dependency is in `Cargo.toml`, where it is used, what we know about its performance and security, and which crates do similar work.

Important:

- This is a source-based review.
- The notes below come from official crate docs, official project docs, Tokio docs, SQLite docs, and RustSec.
- These are not local benchmarks from this repository.
- If a crate does not have a strong official benchmark win, this file says that directly.

Review date:

- 2026-04-14

## Current dependencies

| Crate | Why we use it | Decision |
| --- | --- | --- |
| `ammonia` | Sanitize HTML from feeds | Keep |
| `anyhow` | App error handling | Keep |
| `chrono` | Parse feed dates and work with local day windows | Keep for now |
| `dotenvy` | Load `.env` for local runs | Keep |
| `feed-rs` | Parse RSS and Atom into one data model | Keep |
| `futures` | Run bounded concurrent feed fetch with `buffer_unordered` | Keep |
| `html2text` | Turn feed HTML into readable text | Keep |
| `reqwest` | HTTP client for feeds, OpenRouter, Telegram | Keep |
| `serde` | Core serialization and deserialization | Keep |
| `serde_json` | JSON payloads for APIs and local values | Keep |
| `serde_norway` | YAML parsing for prompts and rules | Keep |
| `sled` | Local key-value store for daily dedupe | Keep for now |
| `tokio` | Async runtime and timers | Keep |

## Removed dependencies

These crates were removed because they were not used or did not justify their cost:

- `moka`
- `rss`
- `tabled`
- `validator`
- `serde_yml`

Reason:

- `moka`, `rss`, `tabled`, and `validator` were not used in real runtime logic.
- `serde_yml` had to go because RustSec says it is `unsound` and `unmaintained`, with no fixed version.

Sources:

- `serde_yml` advisory page: https://rustsec.org/packages/serde_yml.html
- `RUSTSEC-2025-0068`: https://rustsec.org/advisories/RUSTSEC-2025-0068

## Crate by crate

### `ammonia`

Where we use it:

- [src/formatting_text/clean_html.rs](src/formatting_text/clean_html.rs)

Why we use it:

- Feed HTML is external input.
- We sanitize first, then turn it into plain text.

Performance:

- I did not find a strong official benchmark saying `ammonia` is the fastest sanitizer in Rust.
- The value here is safety and correctness, not benchmark bragging.

Security:

- `ammonia` had RustSec advisories in the past.
- Our current version `4.1.2` is in the patched range for the known 2025 advisory.

Crates that do similar work:

- `ammonia`
- custom HTML allowlist logic
- manual stripping, which is usually worse for safety

Decision:

- Keep.

Sources:

- `ammonia` advisories: https://rustsec.org/packages/ammonia.html
- `RUSTSEC-2025-0071`: https://rustsec.org/advisories/RUSTSEC-2025-0071

### `anyhow`

Where we use it:

- app-level error handling across the project

Why we use it:

- This is an application, not a public library.
- `anyhow` is a strong default for app error handling.

Performance:

- No benchmark claim matters much here.
- This crate is about ergonomics and better error context.

Security:

- No specific issue found in the sources checked for this review.

Crates that do similar work:

- `thiserror`
- `eyre`

Decision:

- Keep.

Source:

- `anyhow` docs: https://docs.rs/anyhow/latest/anyhow/

### `chrono`

Where we use it:

- [src/formatting_text/clean_text.rs](src/formatting_text/clean_text.rs)
- [src/fetching_rss/fetch_rss_news.rs](src/fetching_rss/fetch_rss_news.rs)

Why we use it:

- We need RFC 3339 and RFC 2822 parsing.
- We also need local day filtering logic.

Performance:

- I did not find a strong official benchmark saying `chrono` is the fastest current choice.
- `chrono` is acceptable here, but not a proven speed winner.

Security:

- `chrono` had `RUSTSEC-2020-0159`.
- The advisory says it was patched in `>=0.4.20`.
- We use `0.4.44`, so this known issue is already fixed.

Crates that do similar work:

- `chrono`
- `jiff`
- `time`

Decision:

- Keep for now.
- Only replace after a focused date-time cleanup, not for churn.

Sources:

- `chrono` docs: https://docs.rs/crate/chrono/latest
- `chrono` advisories: https://rustsec.org/packages/chrono.html
- `RUSTSEC-2020-0159`: https://rustsec.org/advisories/RUSTSEC-2020-0159.html
- `jiff` docs: https://docs.rs/jiff

### `dotenvy`

Where we use it:

- [src/main.rs](src/main.rs)

Why we use it:

- Simple `.env` loading for local development and local runs.

Performance:

- Startup only.
- No meaningful benchmark discussion needed.

Security:

- The crate is not the main risk.
- The real risk is leaking `.env` values in logs or commits.

Crates that do similar work:

- `dotenvy`
- no crate, only `std::env`

Decision:

- Keep.

Source:

- `dotenvy` docs: https://docs.rs/dotenvy/latest/dotenvy/

### `feed-rs`

Where we use it:

- [src/fetching_rss/fetch_rss_news.rs](src/fetching_rss/fetch_rss_news.rs)

Why we use it:

- We have many external feed sources.
- `feed-rs` handles RSS and Atom in one data model.
- The official docs say it uses `quick-xml` and tries to avoid copying where possible.

Performance:

- I did not find an official benchmark proving `feed-rs` beats every alternative.
- I did find an official benchmark page for `feedparser-rs` claiming very high throughput and explicit parser limits.
- So I cannot honestly say `feed-rs` is the benchmark winner.

Security:

- I did not find a RustSec advisory for `feed-rs` during this review.
- `feedparser-rs` has a stronger official story for parser limits and DoS control.

Crates that do similar work:

- `feed-rs`
- `feedparser-rs`
- `rss`

Decision:

- Keep because the API fit is good and the current code already works.
- If parsing becomes a real hotspot, compare it with `feedparser-rs` using our real feed set.

Sources:

- `feed-rs` docs: https://docs.rs/feed-rs
- `feedparser-rs` docs and benchmarks: https://docs.rs/feedparser-rs

### `futures`

Where we use it:

- [src/fetching_rss/fetch_rss_news.rs](src/fetching_rss/fetch_rss_news.rs)

Why we use it:

- We now use `buffer_unordered` to fetch feeds with bounded concurrency instead of `join_all` on every feed at once.

Performance:

- The official `join_all` docs say that for larger sets of futures, `FuturesOrdered` or `FuturesUnordered` style approaches are often better.
- We fetch 47 feeds, so bounded concurrency is the right move here.

Security:

- No direct security concern in the crate itself for this use.

Crates or patterns that do similar work:

- `futures::stream::buffer_unordered`
- `join_all`
- Tokio task sets and channels

Decision:

- Keep.

Source:

- `join_all` docs: https://docs.rs/futures-util/latest/futures_util/future/fn.join_all.html

### `html2text`

Where we use it:

- [src/formatting_text/clean_html.rs](src/formatting_text/clean_html.rs)

Why we use it:

- Feeds often include HTML summaries or bodies.
- We want text for scoring, grouping, and LLM input.

Performance:

- I did not find an official benchmark win to claim here.
- This is a practical quality choice, not a proven speed champion.

Security:

- It is not our sanitizer.
- We correctly pair it with `ammonia`.

Crates that do similar work:

- `html2text`
- custom HTML stripping

Decision:

- Keep.

Source:

- `html2text` docs: https://docs.rs/crate/html2text/latest

### `reqwest`

Where we use it:

- [src/fetching_rss/fetch_rss_news.rs](src/fetching_rss/fetch_rss_news.rs)
- [src/writing_news/write_news_with_ai.rs](src/writing_news/write_news_with_ai.rs)
- [src/sending_to_telegram/send_to_telegram.rs](src/sending_to_telegram/send_to_telegram.rs)
- [src/main.rs](src/main.rs)

Why we use it:

- This is a good async HTTP client for app code.
- It supports JSON, pooling, request timeouts, compression, and explicit TLS backend choice.

Performance:

- I did not find a strong official benchmark saying `reqwest` is always the fastest client.
- The practical win here is configuration:
  - explicit `rustls`
  - explicit timeouts
  - connection pooling
  - bounded redirect policy

Security:

- We no longer rely on an unbounded client setup.
- We now use explicit request timeouts and explicit TLS backend selection.

Crates that do similar work:

- `reqwest`
- `ureq`
- `hyper`

Decision:

- Keep.

Sources:

- `reqwest` docs: https://docs.rs/crate/reqwest/latest
- `ClientBuilder` docs: https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html

### `serde`

Where we use it:

- data types across the whole project

Why we use it:

- This is the standard Rust ecosystem choice.

Performance:

- No reason to replace it here.

Security:

- The important security question is the data format crate built on top of it, not `serde` itself.

Crates that do similar work:

- `serde`

Decision:

- Keep.

Source:

- `serde` docs: https://docs.rs/serde/latest/serde/

### `serde_json`

Where we use it:

- JSON request and response handling
- local JSON values in app data

Why we use it:

- Standard ecosystem choice for JSON.

Performance:

- I did not find a reason to replace it for this project.
- If extreme JSON speed becomes a hotspot later, then `simd-json` is the comparison point.

Security:

- The main risk is how we log and size payloads, not this crate itself.

Crates that do similar work:

- `serde_json`
- `simd-json`

Decision:

- Keep.

Source:

- `serde_json` docs: https://docs.rs/serde_json/latest/serde_json/

### `serde_norway`

Where we use it:

- [src/app_data/settings/app_env.rs](src/app_data/settings/app_env.rs)
- [src/writing_news/write_news_with_ai.rs](src/writing_news/write_news_with_ai.rs)

Why we use it:

- We need YAML parsing for `news_rules.yml` and `news_message.yml`.
- We replaced `serde_yml` because RustSec says `serde_yml` is unsafe and unmaintained.

Performance:

- I did not find a benchmark win that matters more than the security reason for the switch.

Security:

- The main win here is removing `serde_yml`.
- `serde_norway` is the safer replacement choice for this project.

Crates that do similar work:

- `serde_norway`
- `serde_yaml_ng`
- `serde_yaml2`

Decision:

- Keep.

Sources:

- `serde_norway` docs: https://docs.rs/crate/serde_norway/latest
- `serde_yml` advisory page: https://rustsec.org/packages/serde_yml.html
- `RUSTSEC-2025-0068`: https://rustsec.org/advisories/RUSTSEC-2025-0068

### `sled`

Where we use it:

- [src/reading_news_today/save_and_check_news_read_today.rs](src/reading_news_today/save_and_check_news_read_today.rs)
- [src/reading_stories_today/save_and_check_stories_read_today.rs](src/reading_stories_today/save_and_check_stories_read_today.rs)

Why we use it:

- Embedded key-value store for simple daily dedupe.

Performance:

- `sled` describes itself as a high-performance embedded database.
- But I did not find strong official evidence that it wins across the board.
- In the official `redb` benchmark table, `redb` beats `sled` on multiple workloads.
- So I cannot honestly say `sled` is the benchmark winner.

Security:

- I did not find a RustSec advisory for `sled` during this review.
- The bigger question is long-term maintenance and observability, not a known advisory.

Crates that do similar work:

- `sled`
- `redb`
- `rusqlite`

Decision:

- Keep for now because the workload is tiny and the code is already done.
- If we want a stronger long-term storage choice, compare `redb` and `rusqlite`.

Sources:

- `sled` docs: https://docs.rs/crate/sled/latest
- `redb` docs and benchmark table: https://docs.rs/crate/redb/latest
- `rusqlite` docs: https://docs.rs/rusqlite/latest/
- SQLite serverless: https://www.sqlite.org/serverless.html
- SQLite WAL: https://sqlite.org/wal.html

### `tokio`

Where we use it:

- [src/main.rs](src/main.rs)
- [src/writing_news/write_news_with_ai.rs](src/writing_news/write_news_with_ai.rs)

Why we use it:

- Async runtime and timers for the network-heavy parts of the app.

Performance:

- Tokio is a strong default runtime.
- The real improvement here was not changing runtime, but reducing features:
  - before: `full`
  - now: only `macros`, `rt-multi-thread`, and `time`

Security:

- Smaller feature surface is better than `full` when we do not need the extra APIs.

Crates that do similar work:

- `tokio`
- `async-std`
- `smol`

Decision:

- Keep.

Source:

- Tokio docs: https://docs.rs/tokio/latest/tokio/

## Real performance improvements already applied

1. Removed unused crates from `Cargo.toml`.
2. Replaced unsafe YAML parsing dependency.
3. Stopped using `tokio full`.
4. Added bounded feed fetch concurrency.
5. Added explicit RSS, Telegram, and OpenRouter request timeouts.
6. Switched the HTTP client to an explicit `reqwest::Client::builder()`.
7. Cached `news_message.yml` in memory with `LazyLock` instead of rereading it every loop.

## Remaining tradeoffs

- `feed-rs` is a good fit, but not a proven benchmark winner.
- `sled` is good enough, but not a proven benchmark winner.
- `ammonia` + `html2text` still pull two HTML parser stacks, but they solve two different jobs:
  - sanitize
  - render to text

## Next things worth testing only if performance actually hurts

1. `feed-rs` vs `feedparser-rs` on our real feed set.
2. `sled` vs `redb` vs `rusqlite` on our real dedupe workload.
3. different values for max feed concurrency.
