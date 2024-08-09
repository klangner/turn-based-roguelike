.PHONY: build
build: fmt
	cargo test


fmt: clippy
	cargo fmt --all 

clippy:
	cargo clippy --workspace --all-targets --all-features -- -Dwarnings

run:
	RUST_LOG="warn,roguelike=debug" cargo run