# Rust commands
rust-version:
	echo "Rust command-line utility versions:"
	rustc --version              # Rust compiler
	cargo --version              # Rust package manager
	rustfmt --version            # Rust code formatter
	rustup --version             # Rust toolchain manager
	clippy-driver --version      # Rust linter

install:
	cargo install --path .

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

build:
	cargo build --release

all: format lint test run

help:
	@echo "Available targets:"
	@echo "  rust-version  - Show Rust versions"
	@echo "  rust_install   - Install Rust dependencies"
	@echo "  rust_format    - Format the Rust code"
	@echo "  rust_lint      - Lint the Rust code"
	@echo "  rust_test      - Run tests"
	@echo "  rust_run       - Run the Rust application"
	@echo "  rust_build     - Build the Rust project"
	@echo "  rust_all       - Format, lint, test, and run"