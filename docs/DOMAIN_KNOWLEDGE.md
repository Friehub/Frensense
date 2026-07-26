# Frensense Domain Knowledge Architecture

Because Frensense relies on abstract syntax tree (AST) structures and LSH signatures, it achieves high language-agnosticism. However, some areas of the engine require hardcoded domain knowledge to bridge gaps between frameworks, languages, and semantic meaning.

If the engine is failing to recognize a specific framework's APIs or misclassifying files, you likely need to update one of the following domain knowledge centers.

## 1. Motifs Registry (`src/corpus/motifs.rs`)

**Purpose:** Bridges API naming gaps across different frameworks and languages. 

When a vulnerability pattern in the corpus uses `mysql.query()` but the target codebase uses `db.execute()`, the engine needs to know they are functionally equivalent. Motifs group concrete API calls into abstract semantic categories.

**When to update:**
- If the engine fails to match an injection vulnerability because the target uses a different database driver, HTTP client, or shell executor than the corpus.
- If you add support for a new framework (e.g., adding `helmet` or `csurf` to `SecurityMiddleware`).

**Key Categories:**
- `SqlSink`, `DbQuerySink`
- `CommandExecutionSink`
- `PasswordHashing`
- `SecurityMiddleware`
- `SessionManagement`

## 2. Function Role Classifier (`src/function_role.rs`)

**Purpose:** Assigns high-level architectural roles (`HttpHandler`, `DbQuery`, `ShellExecutor`) to functions based on their API calls as a pre-filtering mechanism before contrastive scoring.

If a candidate is an `HttpHandler` but the corpus pattern is a `ShellExecutor`, the engine immediately rejects the match as structurally incompatible.

**When to update:**
- If the engine is incorrectly filtering out valid vulnerability matches because it misclassified the target function. 
- If you want to add support for a new web framework's routing objects (e.g., adding `ctx` or `c.req` to HTTP variables) or new ORMs.

**Key Hardcoded Arrays:**
- `HTTP_METHODS` (e.g., `send`, `json`, `render`)
- `SHELL_API` (e.g., `exec`, `spawn`, `popen`)
- `DB_API` (e.g., `query`, `execute`, `find`)

## 3. Environment & Context Classification (`src/context/mod.rs`)

**Purpose:** Classifies the overall environment (e.g., `Test`, `Mock`, `RouteHandler`, `Utility`) and data sensitivity of a file based on raw string heuristics in the file path and content.

**When to update:**
- If the engine is failing to identify test files in a new testing framework (e.g., adding `.spec.ts` or `vitest`).
- If you need to detect new types of sensitive data terminology.

**Implementation Details:**
- Uses `path_str.contains()` and `content.contains()` for fast heuristics.
- Look at `FileContext::extract`.

## 4. Parser Normalization (`src/lang/mapper.rs` & `src/fingerprint.rs`)

**Purpose:** Normalizes AST nodes into Frensense's `AbstractKind` to ensure that structurally identical code in different languages produces the identical LSH signature.

**When to update:**
- If structurally identical logic in TypeScript and JavaScript (or Go and Python) is producing different LSH hashes.
- If a parser emits language-specific noise (like `type_annotation` in TS) that needs to be mapped to `AbstractKind::Other` so the fingerprinting algorithm can ignore it.

## 5. Temporal Flow Rules (`src/temporal.rs`)

**Purpose:** Detects violations in stateful sequences (e.g., a `lock()` without an `unlock()`, or an opened connection without a close).

**When to update:**
- If you need to support a new threading or asynchronous library.
- Note: This currently relies heavily on string matching (`call_text.contains(".lock()")`).
