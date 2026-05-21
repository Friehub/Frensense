# GenSense Rule Examples

This directory contains annotated example rules for each major GenSense version.
Use these as starting points when writing custom rules for your project.

```
examples/
├── v0.2.2/          # Diagnostic-only rules (detection, severity, taint analysis)
└── v0.3.0/          # Auto-remediation, CSA project-level rules, SRI-anchored rules
```

## Choosing the Right Format

| Feature                  | v0.2.2 | v0.3.0 |
|--------------------------|--------|--------|
| Pattern matching         | Yes    | Yes    |
| Taint analysis           | Yes    | Yes    |
| Auto-remediation (`--fix`) | No   | Yes    |
| Import injection         | No     | Yes    |
| Project-level CSA rules  | No     | Yes    |
| Symbol-Relative Identity | No     | Yes    |
| Max nesting depth        | Yes    | Yes    |
| Max file lines           | Yes    | Yes    |

## Quick Start

Place rule files inside a `.gensense/rules/` directory at the root of your project:

```
my-project/
└── .gensense/
    └── rules/
        ├── security.yml
        └── architecture.yml
```

GenSense will automatically load all `.yml` files found under `.gensense/rules/`
in addition to the built-in rules.

Run an audit:
```bash
gensense ./my-project
```

Run with auto-remediation (v0.3.0+):
```bash
gensense ./my-project --fix
```

Preview fixes before applying them:
```bash
gensense ./my-project --fix --diff
```
