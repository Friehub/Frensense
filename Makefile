# TaaS Static Auditor Integrity Stack

.PHONY: all audit check test fmt clean help

all: fmt check audit test

## Quality & Safety
check:
	@echo "🔍 Running semantic lints (Clippy)..."
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	@echo "🎨 Enforcing style (rustfmt)..."
	cargo fmt --all -- --check

## Security
audit:
	@echo "🛡️  Checking dependency vulnerabilities..."
	# requires: cargo install cargo-audit
	cargo audit

## Correctness
test:
	@echo "🧪 Running full regression suite..."
	cargo test

test-update:
	@echo "🧪 Updating snapshots..."
	UPDATE_SNAPSHOTS=1 cargo test

## Setup
setup:
	@echo "🔗 Installing local pre-commit hooks..."
	chmod +x scripts/setup-hooks.sh
	./scripts/setup-hooks.sh

## Fuzzing
fuzz-setup:
	@echo "🧪 Setting up fuzzing environment..."
	cargo install cargo-fuzz

fuzz:
	@echo "🧪 Running parser fuzzer..."
	cargo fuzz run audit_parse

## Documentation
doc:
	@echo "📚 Generating institutional documentation..."
	cargo doc --no-deps --open

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

