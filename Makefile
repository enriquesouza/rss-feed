# rss-feed — one command to run everything.
# SQLite is embedded (rusqlite bundled): there is no database server to start.
#
#   make start    → build + start the admin (web UI) and the worker (3h pipeline)
#   make stop     → stop both
#   make status   → what is running + admin health
#   make logs     → follow both logs
#   make coletar  → one fetch round now, fills the news portal (no AI, no Telegram)
#   make resumos  → same, but also writes the AI digest of each story
#   make admin    → start only the web UI
#   make worker   → start only the pipeline

ADMIN_LOG  := /tmp/rss-feed-admin.log
WORKER_LOG := /tmp/rss-feed-worker.log

.PHONY: help build start stop status logs coletar resumos admin worker test

help:
	@grep -E '^#   make' Makefile | sed 's/^#   //'

build:
	cargo build --bin admin --bin rss-feed --bin coletar

start: stop build admin worker status

admin:
	@nohup ./target/debug/admin > $(ADMIN_LOG) 2>&1 &
	@sleep 1
	@echo "admin:  http://localhost:8787  (log: $(ADMIN_LOG))"

worker:
	@nohup ./target/debug/rss-feed > $(WORKER_LOG) 2>&1 &
	@echo "worker: fetching every 3h, posting the digest to Telegram (log: $(WORKER_LOG))"

stop:
	@pkill -f "target/debug/admin" 2>/dev/null && echo "admin stopped" || true
	@pkill -f "target/debug/rss-feed" 2>/dev/null && echo "worker stopped" || true

status:
	@pgrep -fl "target/debug/admin" >/dev/null 2>&1 && echo "admin:  running → http://localhost:8787" || echo "admin:  stopped"
	@pgrep -fl "target/debug/rss-feed" >/dev/null 2>&1 && echo "worker: running" || echo "worker: stopped"
	@curl -s -o /dev/null -w "web:    HTTP %{http_code}\n" http://localhost:8787/ 2>/dev/null || true

logs:
	@tail -n 20 -F $(ADMIN_LOG) $(WORKER_LOG)

coletar:
	cargo run --bin coletar

resumos:
	cargo run --bin coletar -- resumos

test:
	cargo fmt && cargo check && cargo clippy --all-targets && cargo test
