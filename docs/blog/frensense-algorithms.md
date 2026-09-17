---

title: "Frensense Engine - Algorithms and Implementations"

date: 2026-09-17

excerpt: "Every algorithm used in the engine, documented with the actual implementation details, parameter choices, and the reasoning behind each design decision."

---


# Frensense Engine  -  Algorithms and Implementations

Every algorithm used in the engine, documented with the actual implementation details, parameter choices, and the reasoning behind each design decision.

---

## 1. MinHash Signature Computation

**File:** `frensense-engine/src/minhash.rs:43 - 61`

MinHash is used to estimate set similarity without comparing every element. The signature of a set `S` is a vector of `k` minimum values, one per hash function. If two sets have Jaccard similarity `J`, the probability that their signatures agree at position `i` equals exactly `J`. This makes signature agreement a Jaccard estimator.

### Hash Family

The engine uses a **universal multiply-shift hash family** rather than seeded `FxHasher`. The distinction matters: `FxHasher` is not a universal hash family  -  it has correlations between outputs for different seeds that produce biased MinHash signatures.

Each of the `k = 120` hash functions is defined as:

```
h_{a,b}(x) = a·x + b   (mod 2^64)
```

where `a` is forced odd (by ORing with 1) and `(a, b)` are derived from the row index `i` using a mixing step:

```rust
fn minhash_row_hash(value: u64, seed: u64) -> u64 {
    let seed_a = seed.wrapping_mul(0x517cc1b727220a95).wrapping_add(1);
    let seed_b = seed.wrapping_mul(0x9e3779b97f4a7c15);
    let a = seed_a | 1;   // must be odd for 2-universality
    a.wrapping_mul(value).wrapping_add(seed_b)
}
```

The constants `0x517cc1b727220a95` and `0x9e3779b97f4a7c15` are the Fibonacci mixing constants used by many hash functions (the latter is the golden ratio scaled to u64). The first is derived from Knuth's multiplicative hash.

### Transposed Loop

The signature is computed in a **transposed loop**  -  one pass over the input set, updating all 120 minimums at once:

```rust
let mut signature = vec![u64::MAX; 120];
for &h in hashes {
    for (i, min_val) in signature.iter_mut().enumerate() {
        let candidate = minhash_row_hash(h, i as u64);
        if candidate < *min_val {
            *min_val = candidate;
        }
    }
}
```

The alternative  -  one outer loop over rows, one inner loop over elements  -  requires 120 full passes over the input. The transposed version is a single pass, improving cache locality significantly for large fingerprints.

### Empty Set Handling

Empty input returns `vec![0u64; 120]`  -  all-zero signatures. The LSH index treats this as a valid (though degenerate) signature. Two empty sets will always collide in every band and appear as candidates, which is correct (both are empty; similarity is 1.0 by the overlap coefficient and 0.0 by Jaccard convention).

---

## 2. Locality-Sensitive Hashing (LSH)

**File:** `frensense-engine/src/minhash.rs:123 - 208`

LSH converts similarity estimation into a candidate generation problem. Instead of comparing a query fingerprint against all 45k+ corpus entries, the engine hashes the signature into bands and retrieves only items that collide in at least one band.

### Band Parameters and Threshold

The index uses `B = 40` bands of `R = 3` rows each, over `k = 120` total hash functions. The theoretical collision probability at Jaccard `J` is:

```
P(collision in at least one band) = 1 - (1 - J^R)^B
                                  = 1 - (1 - J^3)^40
```

The S-curve inflection point (probability ≈ 0.5) occurs at `J ≈ (1/B)^(1/R) = (1/40)^(1/3) ≈ 0.292`. But in practice the effective working threshold is higher  -  at `J = 0.71`, collision probability exceeds 0.99. Below `J = 0.5` it falls below 0.04. This creates a sharp step that acts as the recall/precision tradeoff knob. The parameters were chosen to give near-certain retrieval above 0.71 while rejecting most below 0.5.

### Band Hashing

Each band takes `R = 3` consecutive positions from the MinHash signature and hashes them together using `FxHasher` (fast, non-cryptographic) to produce a `u64` bucket key:

```rust
let mut hasher = FxHasher::default();
for &val in &signature[band * rows_per_band .. (band+1) * rows_per_band] {
    val.hash(&mut hasher);
}
let bucket_key = hasher.finish();
```

`FxHasher` is appropriate here because the band values are already uniformly distributed (they are MinHash minimums over the multiply-shift family). The `FxHasher` collision risk at the band level is negligible and does not reintroduce the bias that would affect the MinHash step.

### Storage Structure

Each band has its own `FxHashMap<u64, Vec<u64>>` mapping bucket keys to lists of pattern IDs. This is a change from an older fixed-size bucket array implementation: the dynamic HashMap scales naturally with corpus size. At 45k patterns with 40 bands, each band map holds roughly 40k - 44k entries (most items fall into distinct buckets). The `Vec<u64>` per bucket holds all pattern IDs that hashed to that bucket in that band.

### Query

The query walks all 40 bands, computes the bucket key for each, and collects all items from any matching bucket. Results are deduplicated with an `FxHashSet` before returning. A warning is logged when more than 100 candidates are returned (indicating unusually dense corpus regions or a very common fingerprint).

---

## 3. Jaccard Similarity Variants

**File:** `frensense-engine/src/minhash.rs`, `frensense-engine/src/pattern/similarity.rs`

Four distinct implementations exist for different input types and performance requirements.

### `jaccard_similarity`  -  HashSet variant (O(n))

```rust
pub fn jaccard_similarity(a: &FxHashSet<u64>, b: &FxHashSet<u64>) -> f64 {
    let intersection = a.intersection(b).count();
    let union = a.union(b).count();
    if union == 0 { return 0.0; }
    intersection as f64 / union as f64
}
```

Used when inputs are already in hash sets. Returns 0.0 for two empty sets (correct Jaccard convention: undefined → 0.0).

### `jaccard_similarity_sorted`  -  Two-pointer merge (O(n+m))

```rust
pub fn jaccard_similarity_sorted(a: &[u64], b: &[u64]) -> f64 {
    if a.is_empty() && b.is_empty() { return 0.5; }  // Bug #15: should be 0.0 or 1.0
    if a.is_empty() || b.is_empty() { return 0.0; }
    let intersection = intersect_sorted(a, b);
    let union = a.len() + b.len() - intersection;
    intersection as f64 / union as f64
}
```

The sorted variant uses the standard two-pointer intersection algorithm: advance the pointer of whichever array has the smaller current element, increment the intersection count when both elements are equal. Union is computed from the identity `|A ∪ B| = |A| + |B| - |A ∩ B|`, avoiding a full union pass. **Note:** This function contains Bug #15  -  returns 0.5 for two empty inputs instead of a consistent value.

### `signature_similarity`  -  Position-wise agreement

```rust
pub fn signature_similarity(a: &[u64], b: &[u64]) -> f64 {
    let matches = a.iter().zip(b.iter()).filter(|(x, y)| x == y).count();
    matches as f64 / a.len() as f64
}
```

Used to compare MinHash signatures directly. Each position where `a[i] == b[i]` is a Jaccard estimator trial. The fraction of agreement is an unbiased estimator of `J(A, B)` with variance `J(1-J)/k`. At `k = 120` and `J = 0.5`, standard deviation ≈ 0.046  -  sufficient precision for the scoring pipeline.

### `overlap_coefficient_sorted`  -  Containment measure (O(n+m))

```
overlap(A, B) = |A ∩ B| / min(|A|, |B|)
```

Used where one set is expected to be a subset of the other (motif containment, flow-path containment). Returns 1.0 when both are empty, 0.0 when one is empty. This is intentionally different from Jaccard: a small pattern fully contained in a large one scores 1.0 for containment but much less for Jaccard.

---

## 4. LCS-Based AST Edit Distance

**File:** `frensense-engine/src/ast_distance.rs:98 - 136`

The function comment calls this a "greedy approximation of the Zhang-Shasha algorithm" but the actual implementation is **LCS on linearized skeleton sequences**, not Zhang-Shasha. The skeleton is extracted by doing a pre-order DFS of the AST and recording node kinds (excluding leaf identifiers and literals).

### Skeleton Extraction

The DFS skips leaf nodes whose kind is `identifier`, `string`, `number`, `true`, `false`, `null`, `undefined`, or `shorthand_property_identifier`. This makes the skeleton variable-name-invariant and literal-value-invariant  -  two functions with identical structure but different variable names produce identical skeletons.

Node kinds are normalized via the spec's `classify()` method:
- All loop variants (`for`, `while`, `for_in`, `loop_expression`) → `"loop_node"`
- All branch variants (`if`, `switch`, `match`, `ternary`) → `"branch_node"`
- Exception handlers → `"catch_node"`

This normalization makes `for` and `while` loops structurally equivalent in the skeleton, so a refactoring that changes loop type doesn't affect the edit distance.

### LCS Dynamic Programming

The distance is `1 - LCS(A, B) / max(|A|, |B|)`. LCS is computed with the standard space-optimized DP using two rolling rows:

```rust
let mut prev = vec![0usize; n + 1];
let mut curr = vec![0usize; n + 1];

for i in 1..=m {
    for j in 1..=n {
        if a[i-1] == b[j-1] {
            curr[j] = prev[j-1] + 1;
        } else {
            curr[j] = prev[j].max(curr[j-1]);
        }
    }
    std::mem::swap(&mut prev, &mut curr);
    curr.fill(0);
}
```

Space: O(n). Time: O(m·n). For skeletons of typical function size (20 - 100 nodes), this is fast enough to run per candidate without parallelism. The depth guard (bug: allows 257 not 256) prevents pathological O(n²) cases on deeply nested or generated code.

The skeleton hashes are `u64` values  -  each kind string is FxHashed before storage  -  so comparison is integer equality, not string equality.

---

## 5. IDF Weighting

**File:** `frensense-engine/src/corpus/registry.rs:254 - 298`, `frensense-engine/src/fingerprint/`

IDF (Inverse Document Frequency) de-weights n-gram hashes that appear across many corpus patterns, making rare hashes more influential in similarity scoring.

### N-gram IDF

The document set is the collection of all positive fingerprints across all patterns. For each unique n-gram hash `h`:

```
idf(h) = ln(N / df(h))
```

where `N` is the total number of positive fingerprints and `df(h)` is the number of fingerprints containing `h`. This is the standard TF-IDF IDF formula without smoothing.

The weights are stored in `FxHashMap<u64, f32>` and applied at bundle build time. Each fingerprint's `ngram_hashes` is converted to `weighted_ngram_hashes: HashMap<u64, f32>` where the value is `idf(h)`.

At scoring time, when both candidate and target have non-empty `weighted_ngram_hashes`, the weighted Jaccard is used:

```
weighted_J = Σ min(w_i(A), w_i(B)) / Σ max(w_i(A), w_i(B))
```

implemented as:

```rust
let mut intersection = 0.0f64;
let mut union_sum = 0.0f64;
for (h, w) in &candidate.weighted_ngram_hashes {
    union_sum += *w as f64;
    if target.weighted_ngram_hashes.contains_key(h) {
        intersection += *w as f64;
    }
}
for w in target.weighted_ngram_hashes.values() {
    union_sum += *w as f64;
}
intersection / union_sum
```

Note: this is not the exact weighted Jaccard formula  -  it counts the candidate's weight for intersection items but does not take `min(w_A, w_B)`. This is a simplified but effective approximation.

### API-Call IDF

A separate IDF for API call hashes treats each **pattern** (not each fingerprint) as a document. A call appearing in every pattern gets IDF ≈ 0; a call appearing in only two patterns gets high IDF. This weights rare, distinctive API calls more heavily in the API similarity dimension.

```rust
self.api_idf_weights = api_doc_freq
    .into_iter()
    .map(|(call, df)| (call, (total / df).ln()))
    .collect();
```

This is the raw `ln(N/df)` formula. If `df == total` (a call in every pattern), IDF = `ln(1) = 0.0`. The result is used in the scorer to identify the top-3 most-distinctive calls by IDF for the API gate check.

---

## 6. Logistic Regression Weight Learning

**File:** `frensense-bundler/src/pattern/weight_learner.rs`

A 15-parameter logistic regression model per vulnerability category, trained with gradient descent on binary cross-entropy. The model produces a weight vector `w ∈ ℝ^15` that replaces the hand-tuned `DEFAULT_WEIGHTS`.

### Feature Encoding

Raw features are Jaccard/containment similarities in `[0, 1]`. Without transformation, `sigmoid(w·x)` for `w, x ∈ [0,1]^15` produces outputs only in `[0.50, 0.73]`  -  insufficient range for meaningful discrimination. Features are centred and scaled:

```
logit = Σ w_i · (x_i - 0.5) · 4.0
```

With scale factor 4, `x=0` contributes `-2·w_i` and `x=1` contributes `+2·w_i`. The sigmoid of `[-∞, +∞]` covers `[0, 1]`; in practice the outputs span `[0.12, 0.88]`, providing the gradient range needed to push predictions toward 0.0 and 1.0.

### Loss and Gradient

Binary cross-entropy loss for a single example:

```
L = -[y·log(p) + (1-y)·log(1-p)]
```

Gradient w.r.t. `w_i`:

```
∂L/∂w_i = (p - y) · (x_i - 0.5) · 4.0
```

This is computed in the forward pass as `error * (feature - 0.5) * FEATURE_SCALE` and accumulated over all training examples.

### Class Balancing

Positive and negative examples are weighted to avoid majority class domination:

```rust
let pos_weight = 0.5 / n_pos;   // each positive example counts for 1/(2·n_pos)
let neg_weight = 0.5 / n_neg;   // each negative example counts for 1/(2·n_neg)
```

This is equivalent to oversampling: each class contributes exactly 0.5 to the total gradient, regardless of how many examples it has. This is important because corpus patterns often have many more negative than positive examples.

### Anchored L2 Regularization

Standard ridge regularization pushes weights toward zero. The anchored variant pushes toward `DEFAULT_WEIGHTS`:

```rust
grad[i] += L2_LAMBDA * (w[i] - DEFAULT_WEIGHTS[i]);
```

The regularization term is `λ/2 · ||w - w₀||²`. The gradient update is `λ · (w_i - w₀_i)`. With `λ = 0.01` and 400 iterations, the corpus signal dominates when `|w_i - w₀_i|` is large relative to the data gradient. On small corpora, the data gradient is small and the regularizer keeps `w` near the prior. On large corpora, the data gradient dominates and the prior is effectively ignored.

### Training Protocol

1. Initialize `w = DEFAULT_WEIGHTS`
2. Construct training pairs using **leave-one-out best-match**: for each positive `p_i`, find the positive `p_j (j≠i)` that is most similar to `p_i` under `sum(compute_dimensions(p_i, p_j))`. Use that feature vector as the label=1 example. This mirrors the `max-over-positives` logic at inference time.
3. All `(positive, negative)` cross-pairs from the same pattern are label=0 examples.
4. Run 400 gradient descent steps with `lr = 0.05`.
5. Clip weights to `[0, ∞)` after each step (importance weights are non-negative).
6. L1-normalize the final weights so they sum to 1.0.

Categories with fewer than 20 total training pairs fall back to the globally-trained weights. The global fallback itself falls back to `DEFAULT_WEIGHTS` if no training data exists at all.

---

## 7. Per-Pattern Platt Calibration

**File:** `frensense-engine/src/per_pattern_calibration.rs`

Platt scaling fits a sigmoid `P(tp | score) = 1 / (1 + exp(-(A·score + B)))` to map raw similarity scores to calibrated probabilities. Each pattern gets its own `(A, B)` pair.

### Calibration Score

The input to calibration is a single scalar computed as a weighted sum of 11 similarity dimensions (note: only 11 of the 15 scoring dimensions are used; flow-path and motif dimensions are omitted from calibration to avoid circular dependency with the scorer):

```rust
ngram_sim * 0.12 + ast_sim * 0.20 + sig_sim * 0.08 + param_type * 0.04
+ type_usage * 0.03 + semantic * 0.12 + cf * 0.12 + api * 0.12
+ tainted_api * 0.17 + arg_type * 0.04 + literal * 0.04
// Bug #6: these sum to 1.08, not 1.0
```

The calibration score is intentionally simpler than the full scorer to avoid parameter coupling during bundle build.

### Sigmoid

```rust
pub fn calibrate(raw_score: f64, params: Option<&(f32, f32)>) -> f64 {
    let (a, b) = match params {
        Some(&(a, b)) => (a as f64, b as f64),
        None => (8.0, -3.2),   // global fallback
    };
    let z = (a * raw_score + b).clamp(-20.0, 20.0);
    let p = 1.0 / (1.0 + (-z).exp());
    if params.is_none() { p.min(0.55) } else { p }
}
```

The fallback `(8.0, -3.2)` is a hand-tuned sigmoid that reaches 0.5 at `score = 0.4` and 0.73 at `score = 0.55`. The `min(0.55)` cap prevents the fallback from reporting high confidence on patterns with no calibration data.

The `clamp(-20.0, 20.0)` prevents `exp()` overflow: `exp(20) ≈ 5×10^8` is safe; `exp(710)` would overflow a `f64`. The clamp has no practical effect since calibrated scores live comfortably within `[-10, 10]`.

---

## 8. Iterative Dominator Computation

**File:** `frensense-engine/src/cfg/mod.rs:588 - 624`

Dominators are used to determine whether every path from the function entry to a sink block passes through a validation or authentication check. Block `D` dominates block `N` if every path from the entry to `N` passes through `D`.

### Algorithm

The implementation is the classic Cooper/Harvey/Kennedy data-flow algorithm:

**Initialization:**
- `Dom(entry) = {entry}`
- `Dom(n) = all_blocks` for all `n ≠ entry`

**Iteration:**
```
repeat until no change:
    for each block n (in any order):
        Dom(n) = {n} ∪ ∩{Dom(pred) : pred ∈ predecessors(n)}
```

Each iteration intersects the dominator sets of all predecessors and adds `n` itself. The entry block's dominator set `{entry}` is the identity element for intersection (every path must pass through entry, so entry dominates everything). The algorithm converges when no dominator set changes, which takes at most `|V|` iterations for an acyclic CFG and a few more for loops.

### Implementation Detail

The Rust implementation allocates a new `FxHashSet` for each block on each iteration via `(0..n).collect()`, which is expensive for large CFGs. The intersection is computed as:

```rust
let mut new_doms: FxHashSet<usize> = (0..n).collect();  // start = all blocks
for pred in cfg.blocks[i].predecessors.clone() {
    new_doms = new_doms.intersection(&cfg.blocks[pred].dominators).copied().collect();
}
new_doms.insert(i);
```

The `clone()` on predecessors is required because the borrow checker doesn't allow simultaneous immutable and mutable borrows of `cfg.blocks`. This is a performance concern for large functions but correctness-safe.

### Immediate Dominator

`immediate_dominator(cfg, n)` finds the unique block that is the closest dominator of `n` (i.e., dominates `n` but is dominated by all other dominators of `n`):

```rust
let mut idom = cfg.entry;
for &d in doms {
    if d != block_id && cfg.blocks[d].dominators.contains(&idom) {
        idom = d;
    }
}
```

This walks the dominator set and finds the block closest to `n` in the dominator tree by checking whether each candidate dominates the current `idom`. Runs in O(|Dom(n)|²) but dominator sets are typically small.

---

## 9. Reaching Definitions Data-Flow Analysis

**File:** `frensense-engine/src/cfg/def_use.rs:539 - 631`

Reaching definitions is a forward data-flow analysis: at each program point, which variable definitions can "reach" (i.e., flow to) that point without being overwritten.

### Data-Flow Equations

For each basic block `B`:
- `GEN(B)` = definitions produced in `B`
- `KILL(B)` = all definitions of the same variable names as `GEN(B)`, from any other block
- `IN(B)` = `∪{OUT(pred) : pred ∈ predecessors(B)}`
- `OUT(B)` = `GEN(B) ∪ (IN(B) - KILL(B))`

### Implementation

The implementation stores `reaching_defs[block_id]` as a `FxHashSet<usize>` of definition indices. The KILL step is implicit  -  instead of pre-computing kill sets, the algorithm filters incoming definitions by name:

```rust
let def_names: FxHashSet<&str> = block_defs.iter()
    .filter_map(|&idx| chains.definitions.get(idx))
    .map(|d| d.name.as_str())
    .collect();

let mut new_rd: FxHashSet<usize> = incoming
    .into_iter()
    .filter(|&idx| {
        chains.definitions.get(idx)
            .is_none_or(|d| !def_names.contains(d.name.as_str()))
    })
    .collect();
new_rd.extend(&block_defs);
```

This is equivalent to `(IN - KILL) ∪ GEN` computed on each iteration. The outer `while changed` loop runs until the `reaching_defs` sets stabilize.

### Def-Use Link Construction

After reaching definitions converge, each use is linked to its reaching definitions by name:

```rust
for (use_idx, use_) in chains.uses.iter().enumerate() {
    for &def_idx in chains.reaching_defs.get(&use_.block_id) {
        if chains.definitions[def_idx].name == use_.name {
            reaching_defs.push(def_idx);
            chains.use_for_def.entry(def_idx).or_default().push(use_idx);
        }
    }
    chains.def_for_use.insert(use_idx, reaching_defs);
}
```

This produces a bidirectional index: `def_for_use[use_idx]` → which definitions reach this use; `use_for_def[def_idx]` → which uses this definition reaches. The taint engine queries both directions.

---

## 10. BFS Cross-File Taint Propagation

**File:** `frensense-engine/src/data_flow/cross_file.rs`

After per-file taint analysis, functions that expose tainted parameters to callees are registered as taint sources. BFS propagates this taint forward through the call graph up to a depth limit.

### Forward BFS (Source → Callees)

```rust
const PROPAGATE_MAX_DEPTH: usize = 5;

let mut queue = VecDeque::new();
queue.push_back((seed_key.clone(), 0));

while let Some((current, depth)) = queue.pop_front() {
    if depth >= PROPAGATE_MAX_DEPTH { continue; }
    for callee in forward_graph.get(&current) {
        if !visited.contains(callee) {
            visited.insert(callee.clone());
            tainted_sources.insert(callee.clone());
            queue.push_back((callee.clone(), depth + 1));
        }
    }
}
```

The key space is `"{file_path}:{function_name}"` strings. The `forward_graph` maps each function key to the set of functions it calls (extracted from the semantic graph's `Calls` edges). Any function reachable within 5 hops from a confirmed taint source is registered as a tainted source for the next scoring pass.

### Backward BFS (Sink → Callers)

A symmetric backward BFS walks the reverse call graph from sink functions up to depth 5. This identifies all caller functions that eventually reach a taint sink, which are added as candidate findings for the advisory list.

### Sanitizer Pruning

During BFS traversal, if a function key is registered in the sanitizer registry (i.e., it is a known sanitizing function), the BFS does not propagate through it  -  the taint path is considered sanitized and the BFS branch is cut.

---

## 11. Union-Find with Path Compression and Union by Rank

**File:** `frensense-engine/src/engine/clustering.rs:37 - 72`

Used to group structurally similar functions into clusters for the near-duplicate inconsistency analysis (Layer 4).

```rust
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
}

fn find(&mut self, x: usize) -> usize {
    if self.parent[x] != x {
        self.parent[x] = self.find(self.parent[x]);  // path compression
    }
    self.parent[x]
}

fn union(&mut self, x: usize, y: usize) {
    let (xr, yr) = (self.find(x), self.find(y));
    if xr == yr { return; }
    match self.rank[xr].cmp(&self.rank[yr]) {
        Less    => self.parent[xr] = yr,
        Greater => self.parent[yr] = xr,
        Equal   => { self.parent[yr] = xr; self.rank[xr] += 1; }
    }
}
```

**Path compression** in `find()` rewrites the parent pointer of every node on the path to root to point directly to the root. Subsequent `find()` calls on the same node are O(1).

**Union by rank** ensures the tree stays shallow: the root with lower rank always becomes a child of the root with higher rank. The rank is incremented only when two equally-ranked trees merge.

The amortized time per operation is `O(α(n))`, where `α` is the inverse Ackermann function  -  effectively constant for any practical `n`.

### Application to Clustering

After computing MinHash signatures for all functions in the project, the clustering module groups functions by LSH bucket collisions. For every pair `(i, j)` that collide in any LSH band, `uf.union(i, j)` is called. The final clusters are the connected components of this union-find structure. Clusters are then inspected for mixed validated/unvalidated members.

---

## 12. CFG Construction

**File:** `frensense-engine/src/cfg/mod.rs:149 - 430`

The CFG is built in a single cursor-walk over the function's AST. The `LanguageSpec` classifies each node kind into a `NodeRole`, and the builder dispatches on that role to create basic blocks and edges.

### Entry Block

A single entry block is created containing the function root node. The cursor walk proceeds in document order (pre-order DFS via tree-sitter's cursor API).

### Branch Handling

When a `NodeRole::Branch` node is encountered, two new blocks are created immediately: a `branch_block` containing the branch node, and an empty `merge_block` representing the join point. The current block gets two successor edges to both (`Branch` kind). The cursor then descends into the branch body with `current_block = branch_block`. When the cursor returns, `current_block` is restored from `parent_stack`.

This over-approximates the CFG: both the taken and fallthrough paths are connected to the `merge_block`, even for if-else structures where only one path executes. This is sound (conservative) and sufficient for the dominator analysis.

### Loop Handling

`NodeRole::Loop` creates a `loop_body` block with a **back edge** `(loop_body → loop_body, BackEdge)` and an exit edge `(loop_body → after_loop, Unconditional)`. Back edges are tracked separately and not followed during reachability DFS (to avoid infinite loops). Post-dominator computation correctly handles loops because back edges don't change what post-dominates the loop exit.

### Try/Catch/Finally

A try statement creates four blocks: `try_entry`, `catch_b`, `finally_b`, and `try_merge`. The `try_stack` tracks the active try structure so that any exception-throwing node within the try body can add an `Exception` edge to `catch_b`. All three handler blocks converge at `try_merge`.

### Statement Splitting

After the initial CFG is built, `split_statement_blocks` refines it: blocks containing multiple statements are split into one block per statement with `Unconditional` edges between them. This gives the reaching definitions analysis the statement-level granularity needed to precisely determine which definition of a variable is active at each use site.

---

## 13. The Scoring Pipeline  -  Putting It Together

**File:** `frensense-engine/src/pattern/scorer.rs`, `frensense-engine/src/corpus/registry.rs:550 - 810`

The final score for a `(candidate, corpus_pattern)` pair passes through six distinct computational stages.

### Stage 1: LSH Candidate Selection

Two independent LSH indexes are queried (structural + API segments). A third flow-path index is checked separately. Candidates from any index proceed to full scoring; those from both structural and API indexes get a small boost.

### Stage 2: Pre-computation (Parallel)

All 15 similarity dimensions are computed in parallel via Rayon for all `(candidate, corpus_target)` pairs. Results are stored in a `DimCache` keyed by `(candidate_id, target_id)`. This amortizes the computation across multiple pattern matches that share fingerprints.

### Stage 3: Positive and Negative Scoring

For each pattern:
- `best_pos_score = max over all positive fingerprints of weighted_score(candidate, positive)`
- `neg_score = mean over all negative fingerprints of weighted_score(candidate, negative)`
- `raw_score = best_pos_score - neg_discount * neg_score`

The `neg_discount` factor (default 0.4) prevents negatives from fully cancelling positives. A candidate that matches both the vulnerable and fixed versions somewhat still gets a positive score, just attenuated.

### Stage 4: Gate Checks

Before returning a non-zero score, four gates must pass:

1. **Structural overlap gate:** At least one of `{ngram, api, cf, semantic}` similarities must exceed `0.15` against the best positive. Catches the case where a candidate passed LSH but has near-zero similarity on all dimensions.

2. **API-call gate:** Either the best API similarity dimension exceeds `0.20`, or at least one of the top-3 corpus API calls by IDF weight appears in the candidate's API calls. This prevents structurally similar but API-disjoint functions from matching.

3. **Function role gate:** The candidate's computed `FunctionRole` (HttpHandler, DbQuery, ShellExecutor, DataTransformer, Unknown) must be compatible with the pattern's expected role. Incompatible roles: the `FunctionRole` classifier must agree on at least one of the candidate's top-2 roles.

4. **Noise gate (differential signal):** Computes per-dimension signal `s[i] = max(pos_sim[i] - neg_sim[i], 0)`. Passes if `max(s) > 0.40` OR (`count(s > 0.15) >= 3` AND `sum(s) > 0.40`). Prevents matches where the candidate is equally similar to the vulnerable and fixed versions (i.e., the pattern provides no discriminating power).

### Stage 5: Score Adjustments

After gates pass:
- **Freshness penalty:** `score *= freshness_score(pattern_id)` where freshness decays as matches accumulate without taint verification.
- **Taint modifier:** If taint analysis ran and found no taint flow, `score *= 0.6`. If taint flow is confirmed, `score *= 1.1`.
- **Taint branch ratio:** If the function resembles a hollow validator (calls itself validate/check but branches on tainted input < 20% of the time), `score *= 0.5`.
- **Taint verified boost:** If taint is fully verified and branch ratio is high, `score = min(score + 0.08, 0.95)`.

### Stage 6: Calibration

The adjusted score passes through `calibrate(score, pattern_params)` using the per-pattern sigmoid `(A, B)` trained at bundle build time. The output is the `confidence` value reported in the advisory.
