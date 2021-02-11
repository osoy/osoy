build:
	cargo build --release --bin osoy

test:
	cargo test

clean:
	cargo clean

.PHONY: build test clean
