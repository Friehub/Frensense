import json
import os
import re
import sys
import subprocess

JUICE_SHOP_DIR = os.path.abspath("../juice-shop")
FRENSENSE_BIN = "./target/release/frensense"
SCAN_SUBDIRS = ["routes", "lib", "models"]

solve_if_regex = re.compile(r'challengeUtils\.solveIf\(\s*challenges\.([a-zA-Z0-9_]+)')
vuln_files = set()

for subdir in SCAN_SUBDIRS:
    abs_dir = os.path.join(JUICE_SHOP_DIR, subdir)
    if not os.path.isdir(abs_dir): continue
    for root, _, files in os.walk(abs_dir):
        for fname in files:
            if fname.endswith((".ts", ".js")):
                path = os.path.join(root, fname)
                with open(path, "r", encoding="utf-8", errors="replace") as fh:
                    content = fh.read()
                if solve_if_regex.search(content):
                    vuln_files.add(path)

result = subprocess.run([FRENSENSE_BIN, JUICE_SHOP_DIR, "--json"], capture_output=True, text=True)
data = json.loads(result.stdout)
advisories = data.get("advisories", [])

fps = [adv for adv in advisories if adv.get("file_path") not in vuln_files and adv.get("rule_id") == "CORPUS_TS_RACE_CONDITION_READ_CHECK_WRITE"]

with open("fps_race_condition.json", "w") as f:
    json.dump(fps, f, indent=2)

print(f"Dumped {len(fps)} FPs for CORPUS_TS_RACE_CONDITION_READ_CHECK_WRITE")
