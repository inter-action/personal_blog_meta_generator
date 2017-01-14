

# PATH  := node_modules/.bin:$(PATH)
SHELL := /bin/bash

.PHONY: test

test:
	cargo test

test_with_trace:
	RUST_BACKTRACE=1 cargo test

test_with_stdout:
	cargo test -- --nocapture


compile_watch:
	watchexec -e rs -f src cargo build
	
run:
	cargo run

debug_run:
	RUST_BACKTRACE=1 cargo run