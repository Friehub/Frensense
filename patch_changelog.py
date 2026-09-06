import sys

with open('CHANGELOG.md', 'r') as f:
    c = f.read()

addition = """
### Added
- **Codebase Deduplication**: Unified the AST traversals in `loader.rs` and `source_sink.rs`, eliminating hundreds of lines of redundant algorithmic logic.
- **Unified Math Ops**: Extracted Jaccard intersection and scoring evaluation formulas into single shared helpers in `minhash.rs`.
- **Corpus Extension**: Added `ts_juiceshop_idor_update_positive.ts` and `ts_n_plus_one_query` targets to expand the Juiceshop benchmark suite.
- **Documentation**: Moved all top-level documentation `.md` files to the `docs/` folder to clean up the repository root.

"""

c = c.replace('### Added', addition, 1)

with open('CHANGELOG.md', 'w') as f:
    f.write(c)
