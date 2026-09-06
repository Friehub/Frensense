import json
import os
import re
import sys
import subprocess

JUICE_SHOP_DIR = os.path.abspath("../juice-shop")
FRENSENSE_BIN = "./target/release/frensense"
SCAN_SUBDIRS = ["routes", "lib", "models"]
scan_paths = [os.path.join(JUICE_SHOP_DIR, d) for d in SCAN_SUBDIRS if os.path.isdir(os.path.join(JUICE_SHOP_DIR, d))]

solve_if_regex = re.compile(r'challengeUtils\.solveIf\(\s*challenges\.([a-zA-Z0-9_]+)')
vuln_files = set()

for d in scan_paths:
    for root, _, files in os.walk(d):
        for fname in files:
            if fname.endswith((".ts", ".js")):
                path = os.path.join(root, fname)
                with open(path, "r", encoding="utf-8", errors="replace") as fh:
                    if solve_if_regex.search(fh.read()):
                        vuln_files.add(path)

result = subprocess.run([FRENSENSE_BIN] + scan_paths + ["--json"], capture_output=True, text=True)
data = json.loads(result.stdout)

fps = []
for adv in data.get("advisories", []):
    fp_path = adv.get("file_path")
    if fp_path not in vuln_files:
        # shorten path
        short_path = fp_path.replace(JUICE_SHOP_DIR + "/", "")
        fps.append(f"{adv['rule_id']} -> {short_path}:{adv.get('line', '?')}")

with open("19_fps.txt", "w") as f:
    for fp in fps:
        f.write(fp + "\n")
print(f"Extracted {len(fps)} FPs")
