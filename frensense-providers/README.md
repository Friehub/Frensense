# frensense-providers: Compiler & Semantic Integration

`frensense-providers` bridges the gap between shallow syntax trees (Tree-sitter) and deep compiler-level semantics. While `frensense-engine` can operate entirely on AST heuristics, it becomes exponentially more precise when augmented by the exact type-checkers and language servers powering modern toolchains.

## Core Responsibilities

### 1. Compiler Host Integration (`hosts/`)
The providers crate hosts embedded compiler instances directly within the Frensense memory space, allowing it to instantly query symbol resolutions without spinning up heavy external daemon processes.

- **Rust-Analyzer (`providers/rust_hir/`)**: Embeds rust-analyzer's `hir` (High-Level Intermediate Representation) crates to resolve exact trait implementations, macro expansions, and generic type monomorphizations.
- **Oxc / TypeScript (`providers/typescript/`)**: Utilizes the high-performance `oxc` toolkit to achieve near-instantaneous global symbol resolution, module tracking, and exact type inference for JavaScript and TypeScript.

### 2. Deep Semantic Insights
When the Frensense Engine extracts a fingerprint (e.g. `req.body.id`), the AST only knows that it's a `MemberExpression`. The provider allows the engine to query:
1. "Where was this variable originally defined?"
2. "What exact struct/class is `req` bound to?"
3. "Is this type considered a user-controlled input boundary?"

### 3. Graceful Degradation
`frensense-providers` is designed to fail gracefully. 
- If a project fails to compile, is missing dependencies, or uses an unsupported language, the provider simply returns `None` for semantic queries. 
- The `frensense-engine` will immediately fall back to its statistical tree-sitter heuristics, guaranteeing that a scan will always complete even on severely broken codebases.
