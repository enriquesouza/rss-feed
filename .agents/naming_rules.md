# Naming Rules

These rules apply to all names in the project:

- files
- folders
- modules
- functions
- methods
- variables
- constants
- structs
- enums
- traits
- fields
- properties
- classes
- types
- helper names
- temporary names

## Main rule

Keep names as stupid and simple as possible.

- Use easy English.
- Use short, common words.
- Prefer names that sound obvious to Brazilians who are not native English speakers.
- Prefer verbal names.
- Do not try to sound smart, technical, abstract, or elegant.

## Good examples

- `save_news_read_today`
- `save_stories_read_today`
- `read_news_today`
- `group_related_news`
- `find_group_name`
- `check_if_news_is_bad`

## Bad examples

- `persist_processed_articles`
- `hydrate_story_registry`
- `deduplicate_entities`
- `materialize_group_payload`
- `normalize_source_records`

## Extra rule

If there is a choice between:

- a smarter name
- a dumber and more obvious name

always use the dumber and more obvious name.

## Rust Performance Rules

- Measure first. Do not guess. Check performance in `release`, not `dev`.
- Use a profiler before changing hot code.
- Prefer bigger wins first: better algorithm, better data structure, less I/O, less allocation.
- Do not judge runtime performance from `cargo check`, `cargo build`, or debug builds.
- Avoid unnecessary `clone`, `to_string`, `to_owned`, `format!`, `collect`, and temporary `Vec` or `String` in hot paths.
- If the size is known, use `with_capacity`, `reserve`, or `reserve_exact`.
- Reuse `Vec`, `String`, and buffers in loops when possible. Prefer `clear()` over creating a new allocation every time.
- Prefer `&str`, slices, iterators, and references when ownership is not needed.
- If code only collects and then loops again, try to keep it as an iterator instead of building a new collection.
- Use `BufReader` and `BufWriter` for repeated file or socket reads and writes.
- Lock stdout or stderr manually if writing many lines in a loop.
- Keep hot structs small and simple. Large copied types can hurt cache use and can trigger more `memcpy`.
- Only tune Cargo profiles after measuring. If needed, test `opt-level`, `thin` LTO, `codegen-units`, and profiling debug info.
- For profiling release builds, prefer `[profile.release] debug = "line-tables-only"`.
- When naming conversion methods, follow Rust cost rules:
  - `as_` = cheap borrowed view
  - `to_` = work or allocation
  - `into_` = takes ownership

## Rust Security Rules

- Prefer safe Rust. Use `unsafe` only when there is no good safe option.
- Keep every `unsafe` block as small as possible.
- Every `unsafe fn`, `unsafe trait`, `unsafe impl`, `unsafe extern`, and `unsafe` block must explain the safety rule with a `SAFETY:` comment or `# Safety` docs.
- Treat `#[unsafe(no_mangle)]`, `#[unsafe(export_name = ...)]`, and `#[unsafe(link_section = ...)]` as serious footguns. Use them only when truly needed and document why they are sound.
- Validate external input early: file input, network input, env vars, CLI args, RSS text, JSON, YAML, URLs, and dates.
- Prefer type-level validation when possible. If a value must obey a rule, consider a small wrapper type instead of re-checking everywhere.
- Use `Result` for expected failures. Reserve `panic!` for broken invariants, impossible states, and real bugs.
- Do not use `unwrap` or `expect` on untrusted input, network calls, parsing, file I/O, or runtime config.
- `unwrap` and `expect` are acceptable only when the value is truly guaranteed and the reason is obvious and documented.
- Document public failure behavior with `# Errors`, `# Panics`, and `# Safety` when relevant.
- Be careful with process execution. Prefer `Command::new(...).arg(...).args(...)` instead of shell strings.
- Never build shell commands from untrusted input.
- Be extra careful on Windows with `cmd.exe` and `.bat`, because argument parsing can be unsafe with untrusted input.
- Do not casually use `std::env::set_var` or `std::env::remove_var` in multithreaded code.
- Keep dependencies small and audit them regularly.
- Run `cargo audit` to check `Cargo.lock` for known RustSec vulnerabilities.
- Use `cargo-deny` if dependency policy, license policy, duplicate versions, or source policy matter.
- Run `cargo clippy` regularly. Fix `correctness`, `suspicious`, and `perf` problems instead of ignoring them by default.
- Avoid logging secrets, tokens, passwords, cookies, or full private payloads.
- Prefer explicit timeouts, size limits, and retry limits for network and file handling code.

## Rust Source Links

- Rust Performance Book: https://nnethercote.github.io/perf-book/
- Cargo profiles: https://doc.rust-lang.org/cargo/reference/profiles.html
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/checklist.html
- Rust API Guidelines, documentation and failures: https://rust-lang.github.io/api-guidelines/documentation.html
- Rust API Guidelines, validation: https://rust-lang.github.io/api-guidelines/dependability.html
- The Rust Book, error handling: https://doc.rust-lang.org/book/ch09-00-error-handling.html
- The Rust Book, when to return `Result` vs panic: https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html
- Clippy: https://doc.rust-lang.org/clippy/usage.html
- RustSec: https://rustsec.org/
- Rustonomicon: https://doc.rust-lang.org/nomicon/what-unsafe-does.html
- Rust 2024 unsafe attributes: https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-attributes.html
- Rust 2024 newly unsafe functions: https://doc.rust-lang.org/edition-guide/rust-2024/newly-unsafe-functions.html
- `std::process::Command`: https://doc.rust-lang.org/std/process/struct.Command.html
