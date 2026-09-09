import re
content = open("CHANGELOG.md").read()

new_log = """
## [Unreleased] - 2026-09-09
### Fixed
- **Weight Learner Bias Fix**: Fixed a critical gradient descent bug in `frensense-bundler` where missing dimensions (like cross-file `flow_sim`) maintained their `0.5` initialization weight and stole up to 50% of the overall classification weight from valid dimensions during normalization. Dimensions are now initialized to `0.0`.
- **AST Extraction for Semantic Rules**: Replaced the fragile `extract_call_targets` regex in both the bundler and engine with a robust Tree-sitter AST walk, completely eliminating mismatches where the bundler learned structural motifs that the inference engine failed to extract.
- **OOM during LCS similarity**: Replaced the unbounded `O(N*M)` matrix allocation in `lcs_similarity` with an `O(min(N, M))` two-row approach and a length cap, fixing fatal out-of-memory panics when the bundler processed control-flow graphs with 20,000+ paths.
- **Scoring Unification**: Centralized `frensense-bundler` and `frensense-engine` math down to a single `compute_dimensions()` function to prevent divergence in how `tainted_api_sim` and other features are evaluated.

"""

content = content.replace("## [Unreleased]", new_log.strip() + "\n\n## [Unreleased]")
if new_log.strip() not in content:
    content = new_log + content

open("CHANGELOG.md", "w").write(content)
