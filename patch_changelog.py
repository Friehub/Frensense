import re

content = open("CHANGELOG.md").read()

new_entry = """## [Unreleased]

### Fixed
- **Regex Illusion / False Recall**: Identified and resolved a critical bug where `auto_filter` incorrectly extracted `if` and `catch` as exact API calls, artificially inflating both textual overlap (`ngram_sim`, `signature_sim`) and motif hashes across frameworks. The heuristic `contains_call_to` generator in `frensense-bundler` has been temporarily disabled pending an AST-aware replacement.
- **Semantic Override Generalization**: `apply_semantic_override` now successfully triggers on `flow_sim > 0.8` (which hashes abstract `SemanticMarkers` like `SqlSink` rather than raw strings) and `identity_gate > 0.1`, dropping the strict `motif_sim > 0.8` requirement. This allows the engine's core data-flow abstraction to generalize across frameworks (e.g. `db.query` vs `sequelize.query`).
- **Minimum-Score Gate Strictness**: Temporarily disabled the hard-coded gate in `runner.rs` that silently dropped matches if structural/textual overlap (`ngram_sim` AND `signature_sim`) was `< 5%`. This ensures valid data-flow matches between small corpus patterns (15 lines) and large, harness-heavy vulnerable functions (78 lines) are not discarded.

"""

content = re.sub(r"(## \[Unreleased\]\n\n)?### Changed\n- \*\*Consolidated single-binaries", new_entry + r"### Changed\n- **Consolidated single-binaries", content)

open("CHANGELOG.md", "w").write(content)
