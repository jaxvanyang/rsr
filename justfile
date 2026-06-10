build:
	cargo build --release

check:
	cargo build
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

run app *args:
	cargo run --release --bin {{app}} {{args}}
