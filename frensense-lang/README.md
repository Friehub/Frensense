# frensense-lang: Language & Framework Semantics

`frensense-lang` is the knowledge base of the Frensense static analysis engine. 

While Frensense's core engine (`frensense-engine`) is completely language-agnostic and relies purely on mathematical similarity (N-grams, Jaccard distances, structural hashing, and data-flow graphs), `frensense-lang` provides the domain-specific intelligence required to parse code and understand its meaning.

## Responsibilities
- **AST Parsing:** Integrates with Tree-sitter grammars (e.g., `tree-sitter-javascript`, `tree-sitter-rust`) to generate the initial Abstract Syntax Trees.
- **Node Classification:** Implements the `LanguageSpec` trait to tell the engine which AST nodes represent functions, calls, arguments, and control flow structures.
- **The Semantic Registry:** Maps framework-specific API calls to universal semantic tokens so the engine can generalize vulnerabilities across different technology stacks (e.g., understanding that both `prisma.user.findMany` and `mongoose.model.find` are database reads).

---

## Improving the Semantic Registry (Framework Support)

The Semantic Registry allows Frensense to detect vulnerabilities in frameworks it wasn't explicitly trained on. By abstracting specific APIs into universal tokens, the structural matcher can mathematically align code from vastly different frameworks.

### How to add a new framework or API:
The mappings are defined as pure, zero-overhead static Rust arrays in `src/providers/semantics.rs`.

1. Open `src/providers/semantics.rs`.
2. Locate the relevant token (e.g., `SINK_DB_READ`, `SINK_HTTP_OUTBOUND`, `SOURCE_HTTP_REQUEST`).
3. Add the prefix of the framework's API to the slice.

**Example:** Adding `drizzle-orm` to the database read sinks:
```rust
    ("SINK_DB_READ", &[
        "prisma.", 
        "mongoose.model.find", 
        "db.select",       // <--- Added Drizzle ORM read
        "collection.find",
    ]),
```
Because the engine uses `.starts_with()`, mapping `db.select` will automatically catch `db.select().from(users)`.

---

## Adding Support for a New Programming Language

To teach Frensense a completely new language (e.g., Go, Python, or Ruby), you do not need to touch the core engine. You only need to add a new provider here.

### 1. Add the Tree-sitter Grammar
Add the language's Tree-sitter parser to `frensense-lang/Cargo.toml`:
```toml
tree-sitter-go = "0.20.0"
```

### 2. Implement the `LanguageSpec` Trait
Create a new file (e.g., `src/providers/go.rs`) and implement `crate::spec::LanguageSpec`:

```rust
pub struct GoSpec;

impl LanguageSpec for GoSpec {
    fn name(&self) -> &'static str { "go" }
    
    fn extensions(&self) -> &'static [&'static str] { &["go"] }
    
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_go::language()
    }
    
    fn map_api_to_semantic_token(&self, call: &str) -> Option<&'static str> {
        // Map Go APIs like `http.Get` or `sql.DB.Query` to universal tokens
        None
    }

    // Implement structural classifiers (e.g., is_function_node)
    // ...
}
```

### 3. Register the Language
Add your new spec to the `LanguageRegistry` in `src/registry.rs` so the Frensense CLI can automatically route files to it based on their extension.

```rust
registry.register(Box::new(GoSpec));
```

Frensense's `frensense-engine` will immediately be able to fingerprint, cluster, and data-flow analyze the new language!
