# References & Acknowledgments

Frensense is built upon decades of computer science research and relies heavily on the vibrant open-source ecosystem. We owe a massive debt of gratitude to the projects, datasets, and academic principles listed below.

## Open Source Projects

### Tree-sitter
Frensense's multi-language parsing capabilities are entirely powered by [Tree-sitter](https://tree-sitter.github.io/). Its incremental parsing system and standardized AST representations made the multi-scale n-gram fingerprinting possible. 

### Semgrep
The initial idea of treating code as a structural graph rather than raw text was heavily inspired by [Semgrep](https://semgrep.dev/). While Frensense ultimately diverged from handwritten DSLs toward corpus-driven compositional taint, Semgrep's approach to lightweight static analysis paved the way for modern, fast security tools.

### Clippy and ESLint
Frensense is not a replacement for standard linting. Tools like [Clippy](https://github.com/rust-lang/rust-clippy) (for Rust) and [ESLint](https://eslint.org/) (for TypeScript/JavaScript) handle the vast majority of syntactical and stylistic checks flawlessly, allowing Frensense to focus entirely on complex, multi-file semantic analysis.

### The Rust Crate Ecosystem
Frensense's raw speed and mathematical graph routing would be impossible without foundational crates like **[Petgraph](https://crates.io/crates/petgraph)** (which powers the Control Flow and Data Taint graphs), **[Bincode](https://crates.io/crates/bincode)** (for lightning-fast `.frc` serialization), and **[Blake3](https://crates.io/crates/blake3)** (for cryptographic hashing of AST fingerprints).

## AI Assistants & Ideation
While Frensense is a deterministic security engine designed to *catch* AI-generated bugs, the engine itself was built with heavy assistance from AI. We acknowledge the massive productivity and ideation boosts provided by Large Language Models and AI coding assistants. Specifically, **Google Gemini** alongside the **Antigravity** agent, as well as **Anthropic's Claude** and **OpenAI's ChatGPT**, acted as invaluable pair programmers and thought partners during the architectural design, codebase refactoring, and documentation of this project.

## Open Datasets

### CVEfixes
The Frensense Rule Corpus (`.frc`) bundle was seeded and trained using the CVEfixes dataset. We officially attribute and thank the researchers for their foundational work in open-source vulnerability collection:
> **Bhandari, G., Naseer, A., & Moonen, L. (2021).** *CVEfixes: Automated Collection of Vulnerabilities and Their Fixes from Open-Source Software.* In Proceedings of the 17th International Conference on Predictive Models and Data Analytics in Software Engineering (PROMISE '21).

### NVD & MITRE CVE Program
We acknowledge the tireless work of the institutions that maintain the global vulnerability records that Frensense harvests:
> Vulnerability data provided by the **National Vulnerability Database (NVD)**, maintained by the National Institute of Standards and Technology (NIST), and the **Common Vulnerabilities and Exposures (CVE)** list, managed by the MITRE Corporation.

## Computer Science Foundations

### AST N-Grams & Machine Learning for Code
The Layer 1 structural fast-pass leverages concepts heavily researched in the intersection of Machine Learning and Software Engineering (ML4SE). By treating Abstract Syntax Trees as tokenized sequences, Frensense uses **AST N-Grams** and **Jaccard Similarity** to detect the "shape" of a vulnerability. This approach draws conceptual inspiration from embedding models (like *Code2Vec* and *Code2Seq* by Alon et al.), adapting the idea of structural similarity into a deterministic, high-speed heuristic rather than a heavy neural network.

### Compositional Taint Analysis
The core of our Layer 2 DataFlowEngine is based on traditional Taint Analysis. However, to operate at the speed of a linter without exhausting memory, Frensense uses a *compositional* approach. It restricts taint tracing to localized graphs (`taint_max_depth`) and dynamically scales confidence scores based on path entropy, rather than attempting to solve the halting problem across an entire monolithic codebase.

### Control Flow Graphs (CFG)
Under the hood, Frensense uses abstracted Control Flow Graphs to evaluate the temporal ordering of events. This is what allows the engine to detect when a mutex lock is held across an async boundary, or when a channel is sent without releasing its guard.
