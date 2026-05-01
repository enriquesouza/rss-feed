# Rust Performance Rules

> Este projeto é um app de linha de comando que coleta RSS feeds e envia resumos para o Telegram.
> Aplique estas regras com bom senso. Priorize código legível e manutenível sobre micro-otimizações.

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
