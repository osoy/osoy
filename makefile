osoy: src/*.rs src/**/*.rs Cargo.toml
	cargo build --release
	strip target/release/osoy

clean:
	cargo clean
