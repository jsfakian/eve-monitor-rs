# ─── Configurable variables ──────────────────────────────────────────────────
# Runtime directory shared by both processes (socket lives here).
# The monitor also writes its logs to ./persist/monitor/log/ relative to CWD,
# so we run it from LOG_DIR so both sets of logs end up in the same tree.
LOG_DIR  ?= /tmp/eve-monitor-test

# Test-server options
DATA_DIR  ?= $(CURDIR)/test-server/test-data
DEMO_DIR  ?= $(CURDIR)/test-server/demo-data
INTERVAL ?= 200ms

# Export so child processes inherit the value
export XDG_RUNTIME_DIR := $(LOG_DIR)

# ─── Derived paths ───────────────────────────────────────────────────────────
SERVER_BIN  := $(CURDIR)/test-server/test-server
SERVER_LOG  := $(LOG_DIR)/test-server.log
MONITOR_BIN := $(CURDIR)/target/debug/monitor

# ─────────────────────────────────────────────────────────────────────────────

.PHONY: all build build-monitor build-server run-monitor run-server run-demo clean help

## Build both binaries (default)
all: build

## Build both binaries
build: build-monitor build-server

## Build eve-monitor-rs (debug)
build-monitor:
	cargo build

## Build the Go test server
build-server:
	cd test-server && go build -o test-server .

## Run eve-monitor-rs  (monitor logs → $(LOG_DIR)/persist/monitor/log/)
run-monitor: build-monitor
	mkdir -p $(LOG_DIR)
	cd $(LOG_DIR) && $(MONITOR_BIN)

## Run the test server  (server log → $(SERVER_LOG), override: DATA_DIR= INTERVAL=)
run-server: build-server
	mkdir -p $(LOG_DIR)
	$(SERVER_BIN) -d $(DATA_DIR) -interval $(INTERVAL) 2>&1 | tee $(SERVER_LOG)

## Run the test server with the curated demo sequence (1 s interval)
run-demo: build-server
	mkdir -p $(LOG_DIR)
	$(SERVER_BIN) -d $(DEMO_DIR) -interval 1s 2>&1 | tee $(SERVER_LOG)

## Remove build artifacts and logs
clean:
	cargo clean
	rm -f test-server/test-server
	rm -rf $(LOG_DIR)

## Show this help
help:
	@awk '/^##/{msg=substr($$0,4); next} /^[a-zA-Z_-]+:/{print $$1, "-", msg; msg=""}' $(MAKEFILE_LIST)
