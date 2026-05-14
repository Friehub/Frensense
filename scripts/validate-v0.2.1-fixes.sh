#!/bin/bash
# GenSense v0.2.1 Validation Script
# Validates all 3 fixes: W1, W2, W3

set -e

echo "======================================"
echo "GenSense v0.2.1 Fix Validation"
echo "======================================"
echo ""

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

TEST_PASSED=0
TEST_TOTAL=0

run_test() {
    local test_name="$1"
    local test_cmd="$2"
    TEST_TOTAL=$((TEST_TOTAL + 1))
    
    echo -n "🧪 $test_name ... "
    if eval "$test_cmd" > /dev/null 2>&1; then
        echo -e "${GREEN}PASS${NC}"
        TEST_PASSED=$((TEST_PASSED + 1))
    else
        echo -e "${RED}FAIL${NC}"
    fi
}

echo -e "${YELLOW}[W1] BFS Deduplication Fix${NC}"
run_test "  BFS does not deduplicate across files" \
    "cargo test --test project_rules_tests test_bfs_does_not_deduplicate"

echo ""
echo -e "${YELLOW}[W3] E2E Project Rule Tests${NC}"
run_test "  Project rule fires via engine" \
    "cargo test --test e2e_tests test_e2e_project_rule_fires_via_engine"
run_test "  Project rule suppressed by config" \
    "cargo test --test e2e_tests test_e2e_project_rule_suppressed_by_disabled_rules"
run_test "  Project rule severity override" \
    "cargo test --test e2e_tests test_e2e_project_rule_severity_override"

echo ""
echo -e "${YELLOW}[W2] JS API Project Rules (Node.js Integration)${NC}"
run_test "  Node.js auditProject method" \
    "npm test 2>&1 | grep -q 'SUCCESS: Project rules fired via auditProject'"

echo ""
echo "======================================"
echo -e "Results: ${GREEN}$TEST_PASSED/$TEST_TOTAL tests passed${NC}"
echo "======================================"

if [ "$TEST_PASSED" -eq "$TEST_TOTAL" ]; then
    echo -e "\n${GREEN}✅ All v0.2.1 fixes validated!${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Some tests failed${NC}"
    exit 1
fi
