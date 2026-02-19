.PHONY: test lint build

build:
	cargo build --workspace

test:
	cargo test --workspace

lint:
	cargo clippy --workspace -- -D warnings

comply:
	pmat comply check
