#!/bin/bash
set -e

# Frensense Local Integrity Check (CI Simulation)
# This script validates both the Rust engine and Node.js native bindings.

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}==================================================${NC}"
echo -e "${BLUE}        Frensense Local Integrity Pipeline          ${NC}"
echo -e "${BLUE}==================================================${NC}"

# 1. Rust Quality Checks
echo -e "\n${BLUE}[1/5] Running Rust Format Check...${NC}"
cargo fmt --all -- --check
echo -e "${GREEN}✓ Rust format is correct${NC}"

echo -e "\n${BLUE}[2/5] Running Rust Lints (Clippy)...${NC}"
# We use --features cli to avoid NAPI-RS linker issues during clippy
cargo clippy --features cli -- -D warnings
echo -e "${GREEN}✓ Rust lints passed${NC}"

# 2. Rust Core Tests
echo -e "\n${BLUE}[3/5] Running Rust Core & Precision Tests...${NC}"
cargo test --features cli
echo -e "${GREEN}✓ Rust tests passed${NC}"

# 3. Node.js Native Bindings Build
echo -e "\n${BLUE}[4/5] Building Node.js Native Bindings...${NC}"
npm run build
echo -e "${GREEN}✓ Node.js native module built${NC}"

# 4. Node.js Integration Tests
echo -e "\n${BLUE}[5/5] Running Node.js Integration Tests...${NC}"
npm test
echo -e "${GREEN}✓ Node.js integration tests passed${NC}"

echo -e "\n${GREEN}==================================================${NC}"
echo -e "${GREEN}      ALL CHECKS PASSED: Frensense is Stable       ${NC}"
echo -e "${GREEN}==================================================${NC}"
