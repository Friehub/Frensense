content = open("CHANGELOG.md").read()

tp_effect = """
### Benchmark Results
- **False Positives reduced from 18 to 6** (a 67% reduction).
- **True Positives increased from 2 to 3** (unsuppressed `CORPUS_TS_ROLE_HIERARCHY_BYPASS`).
- **Precision increased from 10.00% to 33.33%**.
"""

content = content.replace("math down to a single `compute_dimensions()` function to prevent divergence in how `tainted_api_sim` and other features are evaluated.", "math down to a single `compute_dimensions()` function to prevent divergence in how `tainted_api_sim` and other features are evaluated.\n" + tp_effect)

open("CHANGELOG.md", "w").write(content)
