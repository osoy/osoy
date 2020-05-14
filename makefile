osoy: src/*.rs
	cargo build --release
	strip target/release/osoy

clean:
	cargo clean
