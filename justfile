build:
	cargo build --release

check:
	cargo check
	cargo test --verbose -- --no-capture
	cargo clippy
	cargo fmt --check
	typos
	cargo test --verbose --features use_f64 -- --no-capture

fix:
	cargo fix --allow-dirty
	cargo clippy --fix --allow-dirty
	typos -w
	cargo fmt

run app:
	cargo run --release --bin {{app}}
