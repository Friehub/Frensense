# The Frensense Corpus (.frc)

Starting in `v0.5.0`, Frensense completely abolished the concept of handwritten YAML rules and regex patterns. All analysis is now powered by the **Frensense Rule Corpus (.frc)**.

## What is the `.frc` File?

The `.frc` (Frensense Rule Corpus) bundle is a highly optimized, pre-compiled binary file. It contains the mathematical representations of hundreds of real-world patterns. 

Instead of an engineer writing a brittle rule like:
*If a function calls `db.delete()` and doesn't contain `if (user.isAdmin)`...*

Frensense is fed raw examples of what a vulnerable function looks like (Positive Target), and what a safe function looks like (Negative Target). 

When you run `frensense corpus/targets/ --build-bundle`, Frensense parses all these snippets, extracts their **AST (Abstract Syntax Tree) n-gram fingerprints**, and serializes them using `bincode` and `blake3` into a single, lightning-fast binary blob: `frensense-corpus.frc`.

At runtime, Frensense loads this binary bundle into memory in milliseconds. It never has to re-parse the training data.

## Corpus Content Types

The corpus is not just a vulnerability scanner. By analyzing the AST structures, the `.frc` bundle actively encodes and enforces three distinct categories of code patterns:

1. **Security Vulnerabilities:** Traditional flaws such as SQL Injection, SSRF, Path Traversal, and Race Conditions.
2. **Architectural Invariants (`csa_*`):** Project-specific business logic and structural rules. For example: `auth_no_rejection` (validators that don't return 401s), `missing_payment_gate`, and `find_never_empty`.
3. **LLM Hallucinations (`llm_*`):** AI-generated anti-patterns and sloppy code. For example: `any` parameters, `console.log` in production, or `await` in synchronous functions.

## Pure Code, Zero Config

Frensense requires absolutely **zero manual YAML or TOML configuration** from developers. 

To add a new pattern to the corpus, a developer simply drops two pieces of code into the `corpus/targets/` directory:
1. `pattern_name_positive.ts` - An exact code snippet containing the bad pattern.
2. `pattern_name_negative.ts` - The remediated, safe version of the snippet.

### The `[frensense]` Frontmatter

Frensense does not magically guess how to fix your code. Instead, the developer provides the advisory text directly inside the `_positive` source code file using a simple comment block.

```typescript
// [frensense]
// observation: Writing user-provided data directly to a datastore...
// impact: Any user can overwrite or corrupt data...
// improvement: Call a central auth resolver...

export async function handleDataSync(request: Request, db: Database) {
  // Bad code here...
}
```

During the build phase (`frensense corpus/targets/ --build-bundle`), the engine parses this comment block straight from the AST and bakes it into the `.frc` bundle. Frensense uses this text to generate its rich, deterministic user advisories. The developer never touches a configuration file.

## Semantic Taint Composition

If a structural match is found (meaning your code is "shaped" identically to a known pattern in the corpus), Frensense does not immediately flag it.

First, it passes the node to Layer 2: **The DataFlowEngine**. 
The engine dynamically verifies the Taint Path. It checks if the tainted data flows from a Source (e.g. `req.body`) into a vulnerable Sink (e.g. `db.query()`), without being sanitized.

If your code *looks* like a SQL injection, but the data is safely hardcoded or sanitized, the DataFlowEngine kills the finding instantly. This multi-layered composition guarantees a significant reduction in false positives.
