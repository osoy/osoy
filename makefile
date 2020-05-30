osoy: src/*.rs src/**/*.rs
	cargo build --release
	strip target/release/osoy

clean:
	cargo clean
