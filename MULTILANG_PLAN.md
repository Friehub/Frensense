# Frensense — Multi-Language & Framework Support Plan

---

## The Real Problem First

The gap analysis marked E10 (cross-language pattern abstraction) as `[x]` done.
It is not done. Here is what actually exists:

`collect_structural_markers` in `fingerprint.rs` hashes raw tree-sitter kind strings:

```rust
node.kind().hash(&mut h);   // hashes "function_item" for Rust
markers.insert(h.finish());
```

TypeScript uses `"function_declaration"`, `"method_definition"`, `"arrow_function"`.
Rust uses `"function_item"`, `"impl_item"`.

A Rust positive example and a TypeScript positive example for the same logical pattern
produce **completely different structural marker sets**. They cannot match each other.
Every pattern in the corpus currently has to be duplicated per language — which is
exactly what the corpus already does (`rust_clone_in_loop` vs `ts_*`).

This is fine for now. But it is the root cause of every multi-language problem downstream.
Fixing it is the foundation everything else rests on.

---

## Layer 1 — Abstract Node Kind Taxonomy

A single enum that every language maps into. Raw tree-sitter kind strings disappear
at the boundary; everything above it sees only abstract kinds.

```rust
// frensense-engine/src/lang/kinds.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbstractKind {
    // Declarations
    FunctionDef,       // function_item | function_declaration | method_definition | arrow_function | def
    ClassDef,          // class_item | class_declaration | class_definition
    MethodDef,         // impl function_item | method_definition | def inside class
    InterfaceDef,      // trait_item | interface_declaration | Protocol
    StructDef,         // struct_item | interface_declaration (TS structural) | dataclass
    EnumDef,           // enum_item | enum_declaration | Enum
    ConstDef,          // const_item | const | final field
    ModuleDef,         // mod_item | namespace | module

    // Expressions
    Call,              // call_expression (all languages)
    MethodCall,        // field_expression call | member_expression call
    Await,             // await_expression | .await
    Return,            // return_statement | return_expression
    Assign,            // assignment_expression | let_declaration | variable_declarator
    BinaryOp,          // binary_expression
    UnaryOp,           // unary_expression
    Conditional,       // if_expression | if_statement | ternary_expression
    Loop,              // for_expression | while_expression | loop_expression | for_statement | while_statement
    Match,             // match_expression | switch_statement
    Closure,           // closure_expression | arrow_function (anonymous) | lambda_expression
    TryCatch,          // try block | Result::? | try_statement
    Throw,             // panic! | throw_statement | raise_statement
    Unsafe,            // unsafe_block (Rust-specific, maps to itself)
    Async,             // async_block | async fn | async function

    // Literals & Identifiers
    StringLiteral,     // string_literal | string | template_string
    NumberLiteral,     // integer_literal | float_literal | number
    BoolLiteral,       // boolean_literal | true | false
    Identifier,        // identifier (all languages)
    TypeAnnotation,    // type_annotation | : Type in TS | : type in Python

    // Structure
    Block,             // block | statement_block | body
    Parameters,        // parameters | formal_parameters | parameter_list
    Arguments,         // arguments | argument_list
    ImportDecl,        // use_declaration | import_statement | require
    ExportDecl,        // pub | export | __all__

    // Catch-all
    Other,
}
```

The mapper is a pure function per language:

```rust
// frensense-engine/src/lang/mapper.rs

pub fn abstract_kind(ts_kind: &str, language: Language) -> AbstractKind {
    match language {
        Language::Rust => match ts_kind {
            "function_item" | "closure_expression" if is_method => AbstractKind::MethodDef,
            "function_item"    => AbstractKind::FunctionDef,
            "impl_item"        => AbstractKind::ClassDef,
            "struct_item"      => AbstractKind::StructDef,
            "enum_item"        => AbstractKind::EnumDef,
            "trait_item"       => AbstractKind::InterfaceDef,
            "const_item"       => AbstractKind::ConstDef,
            "mod_item"         => AbstractKind::ModuleDef,
            "call_expression"  => AbstractKind::Call,
            "await_expression" => AbstractKind::Await,
            "return_expression"| "return_statement" => AbstractKind::Return,
            "let_declaration"  => AbstractKind::Assign,
            "if_expression"    => AbstractKind::Conditional,
            "for_expression" | "while_expression" | "loop_expression" => AbstractKind::Loop,
            "match_expression" => AbstractKind::Match,
            "closure_expression" => AbstractKind::Closure,
            "unsafe_block"     => AbstractKind::Unsafe,
            "async_block"      => AbstractKind::Async,
            "block"            => AbstractKind::Block,
            "use_declaration"  => AbstractKind::ImportDecl,
            "string_literal"   => AbstractKind::StringLiteral,
            "integer_literal" | "float_literal" => AbstractKind::NumberLiteral,
            "boolean_literal"  => AbstractKind::BoolLiteral,
            "identifier"       => AbstractKind::Identifier,
            "parameters"       => AbstractKind::Parameters,
            "arguments"        => AbstractKind::Arguments,
            _                  => AbstractKind::Other,
        },
        Language::TypeScript | Language::JavaScript => match ts_kind {
            "function_declaration" | "function" => AbstractKind::FunctionDef,
            "method_definition"    => AbstractKind::MethodDef,
            "class_declaration"    => AbstractKind::ClassDef,
            "interface_declaration"=> AbstractKind::InterfaceDef,
            "enum_declaration"     => AbstractKind::EnumDef,
            "call_expression"      => AbstractKind::Call,
            "await_expression"     => AbstractKind::Await,
            "return_statement"     => AbstractKind::Return,
            "variable_declarator" | "lexical_declaration" => AbstractKind::Assign,
            "if_statement"         => AbstractKind::Conditional,
            "for_statement" | "for_in_statement" | "while_statement" => AbstractKind::Loop,
            "switch_statement"     => AbstractKind::Match,
            "arrow_function"       => AbstractKind::Closure,
            "try_statement"        => AbstractKind::TryCatch,
            "throw_statement"      => AbstractKind::Throw,
            "statement_block"      => AbstractKind::Block,
            "import_statement"     => AbstractKind::ImportDecl,
            "export_statement"     => AbstractKind::ExportDecl,
            "string" | "template_string" => AbstractKind::StringLiteral,
            "number"               => AbstractKind::NumberLiteral,
            "true" | "false"       => AbstractKind::BoolLiteral,
            "identifier"           => AbstractKind::Identifier,
            "formal_parameters"    => AbstractKind::Parameters,
            "arguments"            => AbstractKind::Arguments,
            "type_annotation"      => AbstractKind::TypeAnnotation,
            _                      => AbstractKind::Other,
        },
        // New languages add a new arm here. Nothing else changes.
    }
}
```

**One change in `collect_structural_markers`:**

```rust
// Before (language-specific, non-comparable)
node.kind().hash(&mut h);

// After (language-agnostic, cross-comparable)
abstract_kind(node.kind(), language).hash(&mut h);
```

Now a Rust `function_item` and a TypeScript `function_declaration` both hash to
`AbstractKind::FunctionDef`. A pattern trained on Rust examples matches TypeScript
code without duplication.

---

## Layer 2 — Adding a New Language

Once Layer 1 exists, adding any language is mechanical:

1. Add `tree-sitter-<lang>` as an optional Cargo dependency
2. Add a feature flag in `Cargo.toml`
3. Add a `match` arm in `parser.rs` `get_language()`
4. Add a `match` arm in `mapper.rs` `abstract_kind()`
5. Add symbol and call queries in `get_symbol_query` / `get_call_query`
6. Add corpus examples

Nothing else touches. The scoring, registry, fingerprinting, taint, and temporal
engines are all language-agnostic already.

### Languages to add (priority order)

| Language | Crate | Unlocks |
|---|---|---|
| **Python** | `tree-sitter-python` | Django, FastAPI, Flask, data pipelines |
| **Go** | `tree-sitter-go` | goroutine leaks, context propagation, defer misuse |
| **C / C++** | `tree-sitter-c`, `tree-sitter-cpp` | mutex TOCTOU, AtomicSection (already designed) |
| **Java** | `tree-sitter-java` | Spring, Android, null safety patterns |
| **C#** | `tree-sitter-c-sharp` | ASP.NET, async/await misuse |
| **Ruby** | `tree-sitter-ruby` | Rails N+1, mass assignment |
| **PHP** | `tree-sitter-php` | SQL injection, XSS in templates |
| **Swift** | `tree-sitter-swift` | iOS memory patterns, force-unwrap |
| **Kotlin** | `tree-sitter-kotlin` | Android, coroutine misuse |

All of these have published `tree-sitter` grammar crates. Adding any one takes
a day of work after Layer 1 is done.

---

## Layer 3 — Framework Support

Frameworks are not a language problem. They are a **vocabulary problem** — framework
code uses specific function names, decorators, annotations, and call patterns that
the engine needs to recognize as semantically significant.

The solution is framework-aware corpus examples, not framework-aware engine code.

### How it works

A pattern for a Next.js server action without auth is just a corpus pair:

```
corpus/targets/ts_nextjs_server_action_no_auth_positive.ts
corpus/targets/ts_nextjs_server_action_no_auth_negative.ts
```

```typescript
// positive — no auth check
'use server';
export async function deleteUser(id: string) {
    await db.user.delete({ where: { id } });
}

// negative — auth check present
'use server';
export async function deleteUser(id: string) {
    const session = await getServerSession();
    if (!session) throw new Error('Unauthorized');
    await db.user.delete({ where: { id } });
}
```

The engine learns that `'use server'` + missing `getServerSession` / `auth()` call
is the violation shape. No framework-specific engine code needed.

### Framework corpus sets to build

| Framework | Language | Key patterns |
|---|---|---|
| **Next.js** | TypeScript | Server actions without auth, API routes missing rate limits, `getServerSideProps` with unchecked params |
| **React** | TypeScript | `useEffect` missing cleanup, `dangerouslySetInnerHTML` with user data, hooks called conditionally |
| **Express** | TypeScript/JS | Middleware order (auth before handler), missing `next(err)`, unhandled promise in route |
| **Prisma** | TypeScript | `select *` queries, missing `where` clause on `deleteMany`, transaction without error handling |
| **tRPC** | TypeScript | Mutations without `protectedProcedure`, missing input validation |
| **Tokio** | Rust | `spawn_blocking` inside async without bound, `select!` without timeout arm |
| **Axum** | Rust | Extractors in wrong order, missing auth middleware layer |
| **SQLx** | Rust | Raw query with format!, transaction not committed on all paths |
| **Django** | Python | Missing `@login_required`, raw SQL in views, `DEBUG=True` in production settings |
| **FastAPI** | Python | Missing auth dependency, unchecked `request.body()`, background tasks that swallow errors |

Each framework set is just a directory of corpus pairs. Teams can pin to a specific
framework corpus version independently of the engine version — which was the original
motivation for retiring the embedded rules.

---

## Layer 4 — Cross-Language Patterns

Once the abstract kind taxonomy exists, one set of corpus examples can cover multiple
languages. A pattern like `validate_without_reject` is the same logical violation in
Rust, TypeScript, Python, and Go — the positive example can be written in any language
and will match the equivalent shape in all others.

The naming convention for cross-language patterns drops the language prefix:

```
corpus/targets/validate_without_reject_positive.rs     ← primary example (any language)
corpus/targets/validate_without_reject_negative.rs
```

When the engine scans a TypeScript file, it maps the TypeScript AST to abstract kinds,
then scores against the Rust-trained pattern's abstract kind fingerprint. The match
works because both sides are in the same `AbstractKind` vocabulary.

Language-specific patterns keep the prefix:

```
corpus/targets/rust_transmute_positive.rs       ← Rust only (no TS equivalent)
corpus/targets/ts_as_any_escape_positive.ts     ← TS only
corpus/targets/validate_without_reject_positive.rs  ← cross-language
```

---

## What Changes in the Engine

Only two files change for Layer 1. Everything else is additive.

| File | Change |
|---|---|
| `frensense-engine/src/lang/kinds.rs` | New file — `AbstractKind` enum |
| `frensense-engine/src/lang/mapper.rs` | New file — `abstract_kind(ts_kind, language) → AbstractKind` |
| `frensense-engine/src/fingerprint.rs` | One line — `abstract_kind(node.kind(), language).hash()` instead of `node.kind().hash()` |
| `frensense-engine/src/lang/mod.rs` | New file — `pub enum Language { Rust, TypeScript, JavaScript, ... }` |

Adding a new language after that:

| File | Change |
|---|---|
| `frensense-engine/Cargo.toml` | Add `tree-sitter-<lang>` optional dep + feature |
| `src/parser.rs` + `frensense-engine/src/parser.rs` | Add extension arm in `get_language()` |
| `frensense-engine/src/lang/mapper.rs` | Add `Language::X => match ts_kind { ... }` arm |
| `src/parser.rs` | Add symbol query + call query for new language |
| `corpus/targets/` | Add positive/negative example pairs |

---

## Build Order

### Phase 0 — Abstract Kind Taxonomy (1 day)
Build `lang/kinds.rs` and `lang/mapper.rs`. Update `collect_structural_markers` to
use `abstract_kind`. Run the existing tests — all pass (abstract kinds are a superset,
nothing is removed).

Verify cross-language matching works: load `rust_clone_in_loop_positive.rs` fingerprint,
write an equivalent TypeScript version, confirm structural marker Jaccard is > 0.5.

### Phase 1 — Python (1 day, after Phase 0)
Add `tree-sitter-python`. Map Python kinds. Add corpus pairs for the 5 most common
Python anti-patterns (missing auth decorator, bare `except`, mutable default argument,
`eval()` with user input, hardcoded credentials).

### Phase 2 — Go (1 day)
Add `tree-sitter-go`. Map Go kinds. Corpus: goroutine leak (no `cancel()` on context),
unchecked error return, `defer` inside loop.

### Phase 3 — Framework corpus sets (ongoing)
Framework corpus pairs are data, not code. They can be added continuously without
engine changes. Start with the frameworks your users actually use.

---

## Effort Summary

| Work | Estimate |
|---|---|
| Layer 1 — Abstract kind taxonomy | 1 day |
| Each new language (after Layer 1) | 1 day |
| Each framework corpus set | 0.5–1 day |
| Cross-language pattern corpus | Ongoing, no code cost |

The engine change is small and contained. The ongoing investment is in corpus pairs,
not in engine code — which is the right place for it.
