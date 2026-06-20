# Frensense Integrity Stack

.PHONY: all audit check test fmt clean help corpus-check corpus-gen

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
	-cargo audit

## Correctness
test:
	@echo "[TEST] Running full regression suite (excludes node feature — requires Node.js runtime)..."
	cargo test --all-features

test-update:
	@echo "[TEST] Updating snapshots..."
	UPDATE_SNAPSHOTS=1 cargo test --all-features

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
	@echo "[DOC] Generating Frensense Rule Catalog..."
	cargo run -- --generate-docs

## Security & Compliance
sbom:
	@echo "[SECURITY] Generating CycloneDX SBOM..."
	./scripts/generate-sbom.sh

## Distribution
dist: docs sbom
	@echo "[DIST] Bundling release artifacts..."
	mkdir -p dist
	cargo build --release
	cp target/release/frensense dist/
	cp RULES.md dist/
	cp bom.json dist/
	@echo "[SUCCESS] Release artifacts bundled in dist/"

docker:
	@echo "[DOCKER] Building production image..."
	docker build -t frensense:latest .

## Performance
benchmark:
	@echo "[BENCH] Running performance benchmarks..."
	./scripts/benchmark.sh

## Corpus
corpus-check:
	@echo "[CORPUS] Checking pattern completeness..."
	@python3 scripts/corpus_check.py corpus/targets/

corpus-gen:
	@echo "[CORPUS] Generating missing sidecar .toml files..."
	@python3 scripts/corpus_check.py corpus/targets/ --generate

## Discipline
discipline: check audit test benchmark
	@echo "[DISCIPLINE] All stabilization checks passed."

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

