osoy: src/*.rs src/**/*.rs Cargo.toml
	cargo build --release

strip: osoy
	strip target/release/osoy

clean:
	cargo clean

.PHONY: clean strip
