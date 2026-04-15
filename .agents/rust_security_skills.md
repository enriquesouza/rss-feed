# Rust Security Skills

These skills should be applied by security specialist agents:

- **Network Validation:** Always explicitly check HTTP response status codes (e.g., `response.status().is_success()`) before attempting to parse the body.
- **Payload Limits:** Always enforce strict size limits on external payloads (like HTTP response bodies or file reads) to prevent Out-of-Memory (OOM) and Denial-of-Service (DoS) attacks.
- **Error Handling:** Avoid `unwrap()` and `expect()` on any data originating from an external source (network, user input, files). Use `Result` to propagate errors safely.
- **Timeouts:** All network requests and external I/O operations must have explicit timeouts to prevent hangs.
- **Unsafe Code:** Avoid `unsafe` blocks. If absolutely necessary, encapsulate them and document the safety invariants meticulously with `// SAFETY:` comments.
- **Supply Chain:** Run `cargo audit` regularly and use `cargo-vet` to protect against vulnerable dependencies.