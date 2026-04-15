# Rust Performance Skills

These skills should be applied by performance specialist agents. They incorporate elite, systems-level, and hardcore performance engineering principles.

## Core Principles
- **Architecture First:** Real performance hierarchy is Architecture > I/O model > Memory layout > Locking > Algorithms > Micro-optimizations. Optimize architecture, not just syntax.
- **Profile First:** ALWAYS profile (e.g., `cargo flamegraph`, `perf stat`, `tokio-console`) before optimizing. Never optimize without data.
- **Remove Dead Code:** Audit hot paths regularly. If a computation result (like a DB query) is unused, remove it.

## Zero-Cost & Hot Paths
- **Avoid Unnecessary Allocations:** Use `&str` over `String`, `Bytes` over `Vec<u8>`. Reuse buffers and minimize heap allocations.
- **Pre-allocate Memory:** Use `with_capacity` or `reserve` when the final size is known.
- **Zero-Cost Abstractions:** ALWAYS prefer iterators over manual loops (allows compiler vectorization and elides bounds checks). ALWAYS prefer generics (monomorphization) over `dyn Trait` in hot paths.
- **Const & Inline:** Use `const fn` for compile-time evaluation and `#[inline]` for cross-crate hot functions.
- **Zero-Copy Parsing:** Parse directly from the input buffer without intermediate allocations (e.g., using `simd-json` or `nom`). Avoid `parse → copy → serialize → copy`.
- **Sorting Efficiency:** Use `sort_by_cached_key` when the sorting key extraction is expensive.

## Memory & Cache Locality
- **Avoid Cache Misses:** 90% of latency comes from cache misses, syscalls, and locks. Keep memory contiguous (`Vec<T>` over `LinkedList<T>`).
- **Memory Layout:** Use fixed-size, contiguous, cache-line aligned layouts (`#[repr(C, align(64))]`) for hot path structs. Flat data > pointer chasing.
- **False Sharing:** Pad atomics to cache line boundaries (`crossbeam::utils::CachePadded`) to avoid false sharing between threads.
- **Custom Allocators:** Consider `jemalloc` for long-running servers, `mimalloc` for many small allocs, and `bumpalo` (arena) for request-scoped temp data.

## Concurrency & Async
- **Blocking Async:** Avoid blocking the async runtime for long operations (>10-100μs). For heavy CPU work or synchronous I/O, offload to `tokio::task::spawn_blocking`. However, DO NOT use `spawn_blocking` for operations taking less than 10μs, as the thread context switch overhead will be worse than briefly blocking the runtime.
- **Async vs Threads:** Async is for I/O. Use threads (`rayon`) for bulk CPU-bound work. Hybrid for mixed workloads.
- **Message Passing:** Prefer message passing over shared state. Use SPSC lock-free ring buffers (`rtrb`, `crossbeam`) for ultra-low latency inter-core communication (~10ns).
- **Locking:** Avoid locks in hot paths. Use `RwLock` for read-heavy, `DashMap` for high concurrency. Note: Mutex can outperform lock-free in low-contention scenarios. Use lock-free ONLY when contention is HIGH.
- **Atomics:** Use the weakest atomic ordering possible (e.g., `Relaxed` instead of `SeqCst` when global ordering isn't needed).

## Networking, I/O & Databases
- **Batch Everything:** Batch I/O operations and DB queries. Avoid syscalls in the hot path.
- **Connection Pools:** ALWAYS use a connection pool (`deadpool`, `sqlx`). NEVER use per-request DB connections.
- **Timeouts:** EVERY external HTTP client (`reqwest::Client`) MUST have both connect and request timeouts. No exceptions.
- **Protocol Choice:** Prefer QUIC over WebSocket for non-browser clients (0-RTT, multiplexed). Use Unix sockets for same-machine IPC.
- **Caching:** Pattern is `Request → Cache → DB → Compute → Cache`. Use `moka` for async TTL caching, `dashmap` for in-memory state. Store structs directly to avoid serialization.

## Systems-Level (God-Tier)
- **Pipeline Parallelism:** Design as pipeline stages on separate cores rather than monolithic sequential flows.
- **No Shared State Between Cores:** Cross-core communication invalidates L3 cache. Isolate tasks (e.g., Core 0 for network, Core 1 for parsing).
- **Thread Pinning:** Pin threads to CPU cores (`core_affinity`) for latency-critical work and to respect NUMA node boundaries.
- **Kernel Bypass:** Consider `io_uring`, AF_XDP, or DPDK for line-rate packet processing when latency requirements are <10μs.