# Frensense: Evolving into an Explainable AI Coding Companion

## 1. Executive Summary
The software engineering landscape is shifting toward Agentic AI workflows. However, large language models (LLMs) are probabilistic and often hallucinate vulnerabilities or misinterpret code logic. 
The next frontier for Frensense is to bridge this gap by becoming an **Explainable AI Coding Companion**. By combining Frensense's deterministic, AST-grounded codebase (via `tree-sitter`) with its corpus-driven learning, Frensense can act as the verifiable "immune system" for AI coding agents, providing them with structured, deterministic explanations of vulnerabilities to ensure perfect auto-remediation.

## 2. The Core Concept: Corpus-Driven Explainable AI
Currently, Frensense uses a corpus of `positive` (safe) and `negative` (vulnerable) code samples to calculate semantic fingerprints and detect bugs in zero-day codebases.
To become an AI Companion, Frensense must not only *detect* but also *explain* the bug.

### Architecture Upgrade: The Reasoning Metadata
We will expand the Corpus schema. Alongside every positive/negative pair, we introduce a `reasoning.md` (or JSON metadata) file.
**Example Schema:**
```json
{
  "cwe": "CWE-89",
  "name": "Unsanitized SQL Query",
  "detection_logic": "Data flow from function parameter directly into db.execute() without parameterization.",
  "explanation": "The variable {var_name} flows directly into a database query without sanitization. In our corpus training, this structural pattern resulted in a SQL Injection vulnerability.",
  "remediation_guide": "Use a parameterized query instead of string concatenation. See the corpus positive example for the AST structure of a prepared statement."
}
```

When Frensense calculates a structural match, it extracts the AST context and injects it into this template. The output is a deterministic, highly accurate explanation of *why* the code is wrong, rather than just an error line number.

## 3. The Hybrid LLM + AST Workflow (Agentic Auto-Remediation)
Pure LLM static analysis suffers from high false positives and lack of determinism. We will implement a Hybrid Workflow:

1. **Deterministic Detection:** Frensense scans the repository using `tree-sitter` and semantic fingerprinting. It identifies an exact vulnerable AST node.
2. **Context Extraction:** Frensense extracts the vulnerable function, its type definitions, the `reasoning` metadata, and the safe corpus example.
3. **LLM Handoff (via MCP):** Frensense sends this hyper-focused context to an LLM via the Model Context Protocol (MCP). The LLM is heavily constrained: *“Rewrite this specific AST node to match the safe corpus example. Here is why it is wrong.”*
4. **Deterministic Verification (The Sandbox):** Frensense takes the LLM's suggested diff, applies it in a sandbox, and re-runs the engine. 
    * If Frensense clears the code, the fix is verified.
    * If Frensense still detects the fingerprint, the LLM fix is rejected and re-prompted.
5. **Agentic Action:** The verified fix is committed, and an automated PR is opened containing the deterministic explanation.

## 4. MCP Server Expansion
To integrate with external LLM agents (like Claude or Cursor), Frensense's MCP server must expose new analytical tools:

*   **`explain_violation(file, line)`**: Given a known Frensense error, returns the deterministic reasoning and the relevant AST sub-tree.
*   **`get_corpus_examples(rule_id)`**: Returns the exact safe/unsafe code pairs used to train Frensense on this specific bug.
*   **`verify_sandbox_patch(diff)`**: Allows an LLM to submit a patch and instantly receive a deterministic pass/fail from Frensense's semantic engine before committing.

## 5. Continuous ML-Learning Loop
Frensense will feature a developer feedback loop. When a developer fixes a complex, bespoke bug, they can run:
`frensense learn --vuln bad.rs --safe fixed.rs --reason "Added bounds checking to prevent panic"`

Frensense instantly calculates the new semantic fingerprint and stores it in the corpus. The next time an AI agent makes the same mistake, Frensense immediately catches it and explains it using the developer's exact reasoning.

## 6. The Deterministic vs. Probabilistic Divide
The core philosophy of this architecture hinges on separating concerns between two types of reasoning:
*   **LLM (Probabilistic):** An AI agent is highly creative but probabilistically guesses vulnerabilities ("I *think* this is a SQL injection based on text patterns I've seen"). This leads to hallucinations and missed context.
*   **Frensense (Deterministic):** Frensense operates on mathematical certainty. ("The AST structure here *mathematically matches* the structure of a known vulnerability in our corpus with an exact Jaccard similarity of 92%. It is a bug.")

By letting Frensense handle the deterministic detection and explanation, and letting the LLM handle the creative act of writing the fix based on that explanation, we achieve a flawless, hallucination-free Agentic workflow.

## 7. Open Research Areas (For Personal Exploration)
To fully realize this architecture, there are several advanced research areas that require deeper exploration beyond standard web documentation:

1. **Neuro-symbolic AI Architectures:** Reading full academic papers on systems that combine neural networks (LLMs) with symbolic/deterministic logic (ASTs) to understand the state-of-the-art in bridging these domains.
2. **Local, Open-Weights Models:** Experimenting with small, fast, local models (e.g., Llama 3 8B, DeepSeek Coder) that could be bundled or run locally alongside the Frensense binary to enable entirely offline, zero-latency agentic workflows.
3. **AST Vector Embeddings:** Investigating academic methodologies for converting Abstract Syntax Trees (like those produced by `tree-sitter`) into vector embeddings for advanced ML similarity searches, pushing beyond Frensense's current Jaccard/n-gram fingerprinting.

## 8. Conclusion
By grounding LLM analysis in Frensense's deterministic AST fingerprinting, we solve the reliability problem of AI coding tools. Frensense stops being a passive linter and becomes a continuously learning, explainable **Security Companion** that audits, explains, and verifies AI-generated code.
