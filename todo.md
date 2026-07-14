# Frensense Next Milestone: Semantic Memory Engine (DFG Hashing)

## Objective
Transition Frensense from a boolean taint verifier to a **Semantic Memory Engine**. Instead of relying purely on AST hashing (Layer 1) backed by a simple boolean taint check (Layer 2), we will fingerprint the actual **Data-Flow Graph (DFG) paths** to achieve O(1) "concept matching" that rivals Graph Neural Networks (GNNs) in generalization, but remains deterministic and heavily optimized via Locality Sensitive Hashing (LSH).

## 1. Extract the Taint Path (The "Journey")
Modify `CrossFileVerifier` (and the core data flow tracker) to record the exact path taken from source to sink.
- **Current state**: `verify_taint_flow()` returns `true`/`false`.
- **Target state**: `verify_taint_flow()` returns the ordered sequence of AST nodes / expressions the tainted data flowed through.

## 2. Path Canonicalization (Concept Abstraction)
Map concrete AST operations to abstract semantic concepts to ensure the engine generalizes over syntax variations.
- Build a mapper that translates node types into abstract tokens:
  - `req.body` → `[USER_SOURCE]`
  - `parseInt()` → `[TYPE_CAST]`
  - `trim()` → `[BENIGN_TRANSFORM]`
  - `escape()` → `[SANITIZER]`
  - `db.find()` → `[DATA_SINK]`
- Convert the extracted taint path into a canonical semantic chain: e.g., `[USER_SOURCE] -> [BENIGN_TRANSFORM] -> [STRING_CONCAT] -> [DATA_SINK]`.

## 3. Fingerprinting the DFG (MinHashing Semantic Paths)
Integrate the canonical paths into the LSH engine.
- Generate `data_flow_hashes` from the semantic paths.
- Create N-grams of the semantic path (e.g., bigrams: `[USER_SOURCE]->[BENIGN_TRANSFORM]`, `[BENIGN_TRANSFORM]->[STRING_CONCAT]`).
- MinHash these N-grams and append them to the `FunctionFingerprint`, sitting alongside the existing AST `semantic_markers`.

## 4. Unified Concept Scoring
Update the corpus and `PatternScorer` to evaluate Data-Flow similarity.
- Modify `corpus/targets/` so vulnerability patterns capture their `data_flow_hashes` alongside their AST hashes.
- Update `scorer.rs` so that incoming code evaluates its DFG hash against the corpus's DFG hash using Jaccard similarity.
- **Result**: The engine natively scores "How similar is this data-flow concept to a known SQL injection?", bypassing arbitrary variable naming and framework wrappers.
