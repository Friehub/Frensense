# GenSense Integrity Stack

.PHONY: all audit check test fmt clean help

all: fmt check audit test

## Quality & Safety
check:
	@echo "[CHECK] Running semantic lints (Clippy)..."
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	@echo "[FMT] Enforcing style (rustfmt)..."
	cargo fmt --all -- --check

## Security
audit:
	@echo "[SECURITY] Checking dependency vulnerabilities..."
	# requires: cargo install cargo-audit
	cargo audit

## Correctness
test:
	@echo "[TEST] Running full regression suite..."
	cargo test

test-update:
	@echo "[TEST] Updating snapshots..."
	UPDATE_SNAPSHOTS=1 cargo test

## Setup
setup:
	@echo "[SETUP] Installing local pre-commit hooks..."
	chmod +x scripts/setup-hooks.sh
	./scripts/setup-hooks.sh

## Fuzzing
fuzz-setup:
	@echo "[FUZZ] Setting up fuzzing environment..."
	cargo install cargo-fuzz

fuzz:
	@echo "[FUZZ] Running parser fuzzer..."
	cargo fuzz run audit_parse

## Documentation
docs:
	@echo "[DOC] Generating GenSense Rule Catalog..."
	cargo run --features cli -- --generate-docs

## Security & Compliance
sbom:
	@echo "[SECURITY] Generating CycloneDX SBOM..."
	./scripts/generate-sbom.sh

## Distribution
dist: docs sbom
	@echo "[DIST] Bundling release artifacts..."
	mkdir -p dist
	cargo build --release --features cli,node
	cp target/release/gensense dist/
	cp RULES.md dist/
	cp bom.json dist/
	@echo "[SUCCESS] Release artifacts bundled in dist/"

docker:
	@echo "[DOCKER] Building production image..."
	docker build -t gensense:latest .

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

