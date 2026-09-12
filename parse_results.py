import json

with open('/home/oxisrael/.gemini/antigravity/brain/35f25d0e-aeb4-4ee5-b535-5372ce059f1c/scratch/ecommerce_scan.log', 'r') as f:
    lines = f.readlines()

# Find the start of the JSON output
json_start = -1
for i, line in enumerate(lines):
    if line.startswith('{'):
        json_start = i
        break

if json_start == -1:
    print("No JSON output found")
    import sys
    sys.exit(1)

data = json.loads("".join(lines[json_start:]))
advisories = data.get('advisories', [])

if not advisories:
    print("No advisories found! The codebase is perfectly clean.")
    import sys
    sys.exit(0)

# Group by rule_id
from collections import defaultdict
grouped = defaultdict(list)

for adv in advisories:
    grouped[adv['rule_id']].append(adv)

print(f"Total Advisories Found: {len(advisories)}\n")
for rule, advs in sorted(grouped.items(), key=lambda x: len(x[1]), reverse=True):
    print(f"### {rule} ({len(advs)} findings)")
    for adv in advs[:5]: # print up to 5 examples
        print(f"  - {adv['file_path']} : {adv['enclosing_symbol']} (Conf: {adv['confidence']:.2f})")
    if len(advs) > 5:
        print(f"  - ... and {len(advs) - 5} more")
    print("")

