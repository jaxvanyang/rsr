build:
	cargo build --release

check:
	cargo check
	cargo test --verbose -- --no-capture
	cargo clippy
	cargo fmt --check
	typos

run app:
	cargo run --release --bin {{app}}
