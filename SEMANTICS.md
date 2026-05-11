# GenSense Semantic Engine Laws

This document formalizes the invariants and behaviors of the GenSense Semantic Normalization Layer. All language extractors and analysis engines MUST adhere to these laws to ensure consistent, language-agnostic security auditing.

## 1. The Core Invariants (Engine Laws)

### Law I: Scope Ownership
Every `Binding` and `Assignment` must have clear scope ownership. A `Binding` enters the registry of the current scope, and an `Assignment` must resolve to an existing binding in the current or parent scope.

### Law II: Semantic Determinism
Given the same source code and language grammar, `SemanticExtractor::extract` must ALWAYS produce the same sequence of `SemanticOp` values. The order of operations within a block must reflect the execution order of the source language.

### Law III: Extraction Idempotency
Extracting semantics from a node should not mutate the underlying AST or the global state of the analyzer. The extraction process is a pure function of `(Node, Source, Extension)`.

### Law IV: Traversal Termination
The `DataFlowAnalyzer` must eventually terminate. Recursive block analysis (`EnterBlock`) must be guarded by a maximum depth to prevent stack overflow on malicious or deeply nested code (e.g., auto-generated files).

### Law V: Language Agnosticism
A `SemanticOp` must be "Universally Semantic." If an operation can only be modeled for a single language and has no equivalent in others, it likely belongs in a language-specific lint, not the semantic engine.

## 2. Operation Lifecycles

### Binding
- **Trigger**: Variable declaration (`const`, `let`, `var` in JS; `let` in Rust).
- **Effect**: Registers a name in the `TaintRegistry`. If the value is tainted, the name becomes a source.

### Assignment
- **Trigger**: Mutating an existing variable (`x = y`).
- **Effect**: Transfers taint status from the value to the target variable. If the target was already tainted, it remains tainted (or is updated with the new origin).

### Call
- **Trigger**: Function or method invocation.
- **Effect**: 
    1. Check if the function name matches a configured **Sink**.
    2. Check if any arguments are tainted.
    3. If (Sink Match && Tainted Arg) -> Emit Advisory.

### EnterBlock
- **Trigger**: Entering a new executable region (Function body, loop body, block statement).
- **Effect**: Creates a new scope in the `TaintRegistry`. Operations within the block are analyzed in this scope. Upon exit, the scope is popped.

## 3. Normalization Guidelines

When adding support for a new language construct, ask:
1. **Does it bind a name?** -> Use `Binding`.
2. **Does it change a value?** -> Use `Assignment`.
3. **Does it invoke logic?** -> Use `Call`.
4. **Does it encapsulate execution?** -> Use `EnterBlock`.

If the answer is "None of the above," the construct is likely **Syntactic Sugar** and should be ignored or flattened by the extractor.
