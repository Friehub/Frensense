---
layout: home

hero:
  name: "Frensense"
  text: "Deterministic Security Engine."
  tagline: "Static analysis without the rules. Frensense detects vulnerabilities using concrete code pairs rather than YAML or DSLs—encoding structure, control flow, and data flow simultaneously for exact similarity search."
  actions:
    - theme: brand
      text: Read the Paper
      link: /frensense-paper/00_OVERVIEW
    - theme: alt
      text: Read the Blog
      link: /blog/

features:
  - title: Example-Driven, Not Rule-Driven
    details: Skip writing complex Semgrep rules or regex. Provide a vulnerable function and its fix, and the engine automatically fingerprints the difference.
  - title: Multi-Dimensional Similarity
    details: Fingerprints corpus pairs at build time for fast, exact similarity search at runtime across Rust, TypeScript, and JavaScript.
  - title: Precise Data-Flow
    details: Uses field-sensitive Program Dependence Graphs (PDG) for exact control flow reachability tracking to drastically reduce false positives.
---
