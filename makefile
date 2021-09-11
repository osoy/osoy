build:
	@cargo build --release
	@rm -f osoy
	@mv target/release/osoy osoy

test:
	@cargo test

clean:
	@cargo clean

.PHONY: build test clean
