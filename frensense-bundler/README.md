# frensense-bundler: Corpus Pipeline & Model Training

`frensense-bundler` is the offline pipeline responsible for compiling human-readable vulnerability corpus definitions (`.ts`, `.rs`, `.py`) into an optimized, serialized mathematical representation (`.frc` bundle) consumed by the `frensense-engine`.

## Core Responsibilities

### 1. Corpus Ingestion & Verification (`ingestion/`)
- Parses vulnerability examples (e.g. `ts_sqli_sequelize.ts`) into Tree-sitter ASTs.
- Classifies expected sinks, sources, and data-flows based on custom JSDoc/Docstring tags (e.g., `@frensense-sink`).

### 2. Mutation & Data-Augmentation (`mutators/`)
To maximize recall across wildly different real-world coding styles, the bundler automatically generates structural mutations of the base corpus rules:
- **Async Wrappers**: Wraps patterns in `async`/`await` functions.
- **Try/Catch Blocks**: Injects exception handling wrappers around the vulnerability.
- **Variable Reassignments**: Spreads inline logic out into distinct `let` bindings to fuzz data-flow constraints.

*Note: The Frensense Engine actively deduplicates hits against these mutations at runtime to prevent alert fatigue.*

### 3. Dimensional Weight Learning (`pattern/weight_learner.rs`)
- Frensense does not use hard-coded metric weights. Instead, the bundler runs mini-batch Gradient Descent across the corpus files to learn the optimal feature weights.
- It extracts positive examples (vulnerable functions) and negative examples (secure/sanitized functions), running a calibration loop to assign high weights to distinguishing features (e.g., Taint flow, API similarity) and low weights to generic features.
- A critical bias-fix ensures missing dimensions (like cross-file `flow_sim`) are initialized to `0.0` instead of `0.5`, preventing dimension starvation.

### 4. Serialization (`builder.rs`)
- Packages the patterns, multi-scale hashes, learned weights, and semantic hints into a tightly packed binary `.frc` (Frensense Rule Corpus) file.
- The compiled `.frc` bundle is embedded directly into the Frensense release binary using `include_bytes!`.

### Important Note on Custom Corpora
Because the bundle is embedded via `include_bytes!`, the engine operates strictly on the corpus it was compiled with. Currently, **there is no runtime flag to load an external `.frc` file.** Therefore, if users write their own custom rules and build a new bundle (`frensense --build-bundle`), they **must** recompile the entire Frensense Rust binary (`cargo build --release`) for the engine to recognize and use the new patterns.
