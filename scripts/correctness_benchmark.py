import subprocess
import json
import os

GENSENSE_BIN = "./target/release/frensense"

POSITIVE_TESTS = [
    {
        "file": "tests/correctness/samples/positive/rust_clone_in_loop.rs",
        "expected_rule": "RUST_CLONE_IN_LOOP"
    },
    {
        "file": "tests/correctness/samples/positive/ts_floating_promise.ts",
        "expected_rule": "TS_FLOATING_PROMISE"
    },
    {
        "file": "tests/correctness/samples/positive/secret_leak.rs",
        "expected_rule": "RUST_SECRET_LEAK"
    }
]

NEGATIVE_TESTS = [
    {
        "file": "tests/correctness/samples/negative/rust_clone_clean.rs",
        "unexpected_rule": "RUST_CLONE_IN_LOOP"
    },
    {
        "file": "tests/correctness/samples/negative/ts_awaited_promise.ts",
        "unexpected_rule": "TS_FLOATING_PROMISE"
    }
]

def run_gensense(file_path):
    result = subprocess.run([GENSENSE_BIN, file_path, "--json"], capture_output=True, text=True)
    if result.returncode != 0:
        return []
    try:
        return json.loads(result.stdout)
    except:
        return []

def main():
    print("# GenSense Correctness Benchmark")
    print(f"| Test Case | Expected | Result | Status |")
    print(f"| :--- | :--- | :--- | :--- |")

    true_positives = 0
    false_positives = 0
    true_negatives = 0
    false_negatives = 0

    for test in POSITIVE_TESTS:
        advisories = run_gensense(test["file"])
        rule_ids = [a["rule_id"] for a in advisories]
        if test["expected_rule"] in rule_ids:
            status = "✅ PASS"
            true_positives += 1
        else:
            status = "❌ FAIL (FN)"
            false_negatives += 1
        print(f"| {test['file']} | {test['expected_rule']} | {', '.join(rule_ids) if rule_ids else 'None'} | {status} |")

    for test in NEGATIVE_TESTS:
        advisories = run_gensense(test["file"])
        rule_ids = [a["rule_id"] for a in advisories]
        if test["unexpected_rule"] not in rule_ids:
            status = "✅ PASS"
            true_negatives += 1
        else:
            status = "❌ FAIL (FP)"
            false_positives += 1
        print(f"| {test['file']} | NOT {test['unexpected_rule']} | {', '.join(rule_ids) if rule_ids else 'None'} | {status} |")

    print("\n## Summary Metrics")
    total = len(POSITIVE_TESTS) + len(NEGATIVE_TESTS)
    precision = true_positives / (true_positives + false_positives) if (true_positives + false_positives) > 0 else 0
    recall = true_positives / (true_positives + false_negatives) if (true_positives + false_negatives) > 0 else 0
    
    print(f"- **Accuracy**: {(true_positives + true_negatives) / total:.2%}")
    print(f"- **Precision**: {precision:.2%}")
    print(f"- **Recall**: {recall:.2%}")
    print(f"- **False Positive Rate**: {false_positives / len(NEGATIVE_TESTS):.2%}")

if __name__ == "__main__":
    main()
