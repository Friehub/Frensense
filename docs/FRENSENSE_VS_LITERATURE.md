# Frensense vs. the Academic Literature
## A Comparison Based on Reading the Papers

Sources read:
- Kaniewski et al. (2025), *"A Systematic Literature Review on Detecting Software Vulnerabilities with LLMs"*, arXiv:2507.22659 — 227 studies, January 2020–June 2025
- Referenced contrastive learning papers (Chen et al. ICSE 2024, SCL-CVD, VDCRL)

---

## What the Literature Actually Does With Contrastive Pairs

The phrase "contrastive pairs" appears in two entirely different contexts in
the academic literature and in Frensense. Reading the paper carefully reveals
they are not the same technique.

### In the academic literature

The 2025 systematic review describes contrastive pairs as one strategy for
**few-shot prompting** of LLMs:

> *"Constructing contrastive pairs, where both a vulnerable code snippet and
> its corresponding fixed version are provided within the same prompt."*
> — Section 4.4.1, Kaniewski et al. 2025

The pairs are placed inside the LLM's context window as examples. The LLM
reads them and is expected to use them to calibrate its response about a third,
unseen snippet. The vulnerable and fixed versions are **prompt content**, not
a scoring mechanism. The LLM still makes the final decision probabilistically.

The contrastive learning papers (ICSE 2024, SCL-CVD, VDCRL) use pairs
differently: as **training data** for neural models. The neural model is
trained to pull representations of vulnerable code together in embedding space
and push representations of safe code apart. At inference time, the model takes
a new snippet and produces an embedding, which is compared to the learned
cluster boundaries.

### In Frensense

Frensense uses pairs as a **reference corpus scored at inference time**:

1. A new function is fingerprinted (n-grams, AST skeleton, API call hashes,
   control flow sequence, motif hashes)
2. Its fingerprint is scored against every positive (buggy) example in the
   corpus — produces `similarity_positive`
3. Its fingerprint is scored against every negative (fixed) example in the
   corpus — produces `similarity_negative`
4. Final score = `f(similarity_positive, similarity_negative)` where high
   negative similarity penalises the final score

No LLM is involved at any point. No neural model is involved. The scoring
is Jaccard similarity on sorted hash arrays — a deterministic integer operation.

The negative corpus in Frensense is not a prompt hint to a model. It is a live
constraint that downgrades the score of anything that looks like safe code.
This is why a function calling `exec("ls")` (constant, safe) scores differently
from `exec(req.body.cmd)` (user-controlled, dangerous) even though both contain
`exec` — the fixed corpus contains the constant-argument form.

```
Academic contrastive pairs              Frensense contrastive pairs
────────────────────────────────        ──────────────────────────────────
Used as:                                Used as:
  Prompt examples (few-shot)              Reference corpus (scored at runtime)
  OR training data for neural model

Role of negative example:              Role of negative example:
  Shows LLM "this is what safe          Scored simultaneously with positive
  code looks like" in the prompt        Penalises candidates that resemble
  (hint, not constraint)                safe code (hard constraint on score)

Decision mechanism:                     Decision mechanism:
  LLM probability estimate              Jaccard similarity on hash arrays
  (probabilistic, varies per run)       (deterministic, same every run)

Hardware required:                      Hardware required:
  GPU for viable latency                 CPU only, milliseconds per function

Output:                                 Output:
  "Vulnerable: yes/no" (binary)          Confidence score [0.0, 1.0]
  OR probability P(vulnerable)           with per-dimension breakdown
```

---

## What the Paper Found About the State of the Field

The systematic review analysed 227 studies. These findings are directly
relevant to understanding where Frensense sits:

### 1. Language coverage

<cite index="45">The surveyed studies predominantly address C and C++ vulnerabilities.</cite>

The dominant datasets — Big-Vul (Fan et al. 2020), Devign (Zhou et al. 2019),
D2A, ReVeal — are almost entirely C/C++. Frensense targets TypeScript, JavaScript,
Go, and Rust — the languages of modern web development and systems programming
that appear in very few of the 227 academic studies. This is a genuine gap the
corpus fills, not a reiteration of existing work.

### 2. Task formulation

<cite index="45">Binary classification determines whether a given code contains a security
vulnerability or not ('Yes'/'No'). With 156 studies, binary classification
remains the most prevalent task formulation.</cite>

Binary classification tells you the code is vulnerable. It does not tell you
how confident to be, which dimension of similarity triggered the finding, which
specific corpus example it resembles, or whether the input looks more like the
buggy or the fixed version. Frensense produces a continuous confidence score
with a per-dimension evidence breakdown — closer to the 27 studies that do
vulnerability-specific classification, but with explicit negative-corpus
penalisation that none of the 227 studies implement as a scoring component.

### 3. Interpretability is an open problem

The paper identifies L6 as a current limitation:

> *"L6 Model Interpretability and Explainability: It remains difficult for
> security engineers to understand why a model classified a code snippet as
> vulnerable. Most approaches rely on attention-based or gradient-based
> explanations, which are post-hoc and not always reliable."*

Frensense's evidence block — showing exactly which API calls matched, what
motif was recognised, what the per-dimension similarity scores were, and which
positive corpus example produced the best match — is a direct implementation
of what the review identifies as missing across all 227 academic studies.
The explanation is not post-hoc. It is a byproduct of the scoring computation.

### 4. Pipeline integration is an open problem

The paper identifies L7 as a current limitation:

> *"L7 Integration into Pipelines and Workflows: Studies predominantly focus
> on offline evaluation using pre-collected datasets. Integration into developer
> workflows and automated CI/CD pipelines remains a challenge. The high false
> positive rates of many approaches, as well as their non-deterministic outputs,
> make it difficult to establish reliable automated pipelines."*

This is stated explicitly in the review. All 227 academic studies were evaluated
on offline datasets. None were designed for or evaluated in CI pipeline
integration. The review authors specifically flag non-determinism as the reason
this is hard. Frensense's deterministic hash-based scoring is the engineering
decision that makes CI integration viable. The review identifies the problem.
Frensense implements a solution to it.

### 5. Dataset quality is an open problem

> *"L2 Dataset Labeling and Quality: Widely used datasets, such as Big-Vul and
> Devign, have been shown to have high duplication rates, data imbalance,
> biased vulnerability distribution, and label noise."*

The academic literature uses publicly available datasets collected from CVE
repositories. These contain code at the commit level (entire file diffs) rather
than at the function level. Labels are often wrong because the CVE-associated
commit may fix multiple issues, fix unrelated issues, or the labelling process
introduces noise. The review cites multiple papers documenting this.

Frensense's corpus is manually curated at the function level: each pair is one
specific buggy function and its specific fix. The corpus quality guide sets
minimum standards (the quality score checklist) and Frensense Hub's automated
validation confirms the engine fires on positives and not on negatives before
accepting submissions. The datasets the academic literature uses do not have
equivalent quality gates.

### 6. LLM reasoning struggles with subtle differences

Steenhoek et al. (2025), one of the 227 surveyed studies, is specifically cited
for this finding:

> *"Steenhoek et al. demonstrate that the models struggle to reason about the
> code semantics relevant to identifying vulnerabilities, especially subtle
> semantic differences caused by small textual changes."*

This is the core failure mode. A function calling `exec(cmd)` where `cmd` came
from `req.body` is vulnerable. The same function where `cmd` came from a
hardcoded constant is not. The textual difference is small. LLMs struggle with
this. Frensense's data-flow path fingerprint (`UserInputSource → sink` chain)
is specifically designed to encode this distinction as a hash dimension, making
it a scored feature rather than a reasoning task.

### 7. No runtime confirmation exists anywhere in the 227 studies

The review covers 227 studies from January 2020 to June 2025. None of them
include runtime behavioral confirmation as part of the detection pipeline.

The review notes that some studies include "Security Testing" as an objective
(Wang et al. 2025a) and validates results by "running the security tests
generated." But these tests are LLM-generated test cases, not probe-based
behavioral oracles. The architecture of sending a targeted HTTP probe derived
from the static pattern, observing the response with a canary oracle, and
using the oracle result to confirm or deny the static finding — this
combination does not appear in any of the 227 studies.

---

## Precise Differences Table

| Dimension | Academic literature (227 studies) | Frensense |
|---|---|---|
| **Detection mechanism** | Neural model (embedding similarity or classification probability) | Hash-based Jaccard similarity on sorted arrays |
| **Determinism** | Probabilistic — same code, different output is possible | Deterministic — identical output every run, every machine |
| **How negative examples are used** | As prompts (few-shot) or training data for embeddings | As live scoring constraint — high negative similarity penalises the finding |
| **Hardware** | GPU required for viable latency | CPU only, milliseconds per function |
| **Primary language** | C/C++ (dominant datasets: Big-Vul, Devign) | TypeScript, JavaScript, Go, Rust |
| **Output** | Binary (vulnerable/not) or probability P(vulnerable) | Continuous confidence score + per-dimension evidence breakdown |
| **Explainability** | Post-hoc (attention, gradients) — identified as L6 open problem | Native — evidence block is byproduct of scoring |
| **CI pipeline integration** | Not designed for it — identified as L7 open problem | Primary use case; designed for millisecond CI execution |
| **Dataset quality** | CVE-derived, known duplication and label noise (L2 open problem) | Function-level, manually curated, automated quality gates |
| **Runtime confirmation** | Does not exist in any of 227 studies | Frensense Runtime: targeted HTTP probe + behavioral oracle |
| **Subtle semantic differences** | Identified as open challenge (Steenhoek et al. 2025) | Data-flow path fingerprint encodes this as a hash dimension |

---

## Where Frensense Is Not Novel

Being precise requires stating this clearly:

**The corpus pair concept** — taking a buggy function and its fixed version as
training signal — is in the literature since at least 2021. Big-Vul and CVEfixes
are built on this concept. The contrastive learning papers (ICSE 2024, SCL-CVD,
VDCRL) use this concept with neural models.

**AST-based fingerprinting** — extracting structural features from code using
tree-sitter or similar parsers — appears in dozens of papers including early
Devign work (2019).

**MinHash LSH for code similarity** — used in clone detection research since
the early 2000s.

**The motivation** — reducing false positives in static analysis — is the
stated goal of most of the 227 papers.

---

## Where Frensense Is Different

**The combination in a deterministic production system:**

The 227 academic papers all share one property: the LLM or neural model is the
final arbiter. Frensense removes the model from the inference path entirely.
The corpus is fixed at build time. The scoring is hash comparison. There is no
model to be non-deterministic, no GPU to be unavailable in CI, no embedding to
drift between versions.

**The negative corpus as a simultaneous scoring constraint:**

In academic few-shot approaches, the negative example is a hint. In academic
contrastive training, the negative example shapes the embedding space. In
Frensense, the negative example is scored against every candidate at runtime
and directly penalises the final score. This is a different computational role.
The academic literature does not implement this as a live scoring constraint.

**Runtime confirmation:**

The 2025 systematic review covers five years of research and 227 papers. None
include a corpus-derived probe generator that sends targeted HTTP requests,
observes behavioral oracles, and feeds results back as confirmed/unconfirmed
findings. This pipeline does not exist in the academic literature in any form.

**Language coverage:**

The academic datasets are C/C++ dominated. Frensense targets the languages
that modern web developers and systems programmers actually use: TypeScript,
JavaScript (Express, Fastify, Next.js, Remix, NestJS, Hono), Go (net/http,
Gin, Echo), and Rust (Axum, Actix-web). The gap between the academic dataset
landscape and the production codebase landscape is real and documented by the
survey itself.

**What the survey explicitly says is still missing** (L6, L7) is what
Frensense implements. That is the gap.
