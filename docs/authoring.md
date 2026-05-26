# Rule Authoring & Schemas

GenSense rules are defined in YAML. To make authoring easier and less error-prone, we provide formal JSON schemas and LLM-powered generation tools.

## LLM-Powered Generation

If you use an LLM (like Claude 3.5, GPT-4o, or Gemini 1.5) to write rules, we recommend using our **GenSense Rule Expert** system prompt. This prompt provides the model with the necessary AST context and schema constraints.

- [View the GenSense Rule Expert Prompt](./prompts/rule-expert.md)

## JSON Schemas

You can link these schemas in your IDE (e.g., VS Code via the YAML extension) to get real-time validation and IntelliSense.

| Version | Description | Schema Link |
|---------|-------------|-------------|
| **v0.3.1** | **Latest.** Adds SPG graph context, algebraic flow combinators, CSA body analysis (`body_must_contain`, `body_may_delegate_via`), and 6 tuning parameters. | [v0.3.1 Schema](./schema/gensense-v3.schema.json) |
| **v0.3.0** | SRI, CSA project rules, auto-remediation. | [v0.3.0 Schema](./schema/gensense-v3.schema.json) |
| **v0.2.2** | Legacy diagnostic-only format. | [v0.2.2 Schema](./schema/gensense-v2.schema.json) |

### Integrating with VS Code

To enable IntelliSense for your custom rules, add the following to your `.vscode/settings.json`:

```json
{
  "yaml.schemas": {
    "./docs/schema/gensense-v3.schema.json": ".gensense/rules/*.yml"
  }
}
```

## Version Comparison

### v0.2.2 (Diagnostic)
- Purely diagnostic findings.
- Pattern matching (`if_matches`).
- Taint analysis (`source_pattern`/`sink_pattern`).
- Size and nesting limits (`max_lines`/`max_depth`).
- Used `domain` field for categorization (now `category` with backward-compatible alias).

### v0.3.0 (Remediation & Governance)
- **Everything in v0.2.2**, plus:
- **Auto-Remediation**: `fix_pattern` and `fix_with` templates.
- **Import Injection**: `inject_import` with `{{root}}` resolution.
- **Project-Level Guards**: `must_have_guard` for cross-file CSA verification.
- **Symbol-Relative Identity**: Findings are anchored to functions/classes, not line numbers.
- **Schema Contracts**: `schema_contract` block validates source patterns against database schema definitions (Prisma). Fields: `source_ext`, `source_pattern`, `source_file_glob`, `schema_type` (Prisma), `schema_glob`, `schema_extract` (ModelNames/FieldNames/EnumValues).
- **YAML Version Field**: Optional `version: "0.3.0"` at the top of rules files.

### YAML Format Changelog

| Version | Changes |
| :--- | :--- |
| **0.3.1** | Added `body_must_contain`, `body_may_delegate_via` for CSA body analysis. Added 6 algebraic flow combinator fields: `across_boundary`, `all_of`, `any_of`, `not`, `without_constraint`, `without_exclusion`, `chain_source`, `chain_through`, `chain_sink`. Added tuning fields: `max_source_lines`, `ngram_window_size`, `min_ngram_count`, `taint_confidence_interprocedural`, `taint_confidence_intraprocedural`, `default_taint_max_depth`. |
| **0.3.0** | Added `version` field. Renamed `domain` → `category` (backward compat alias kept). Added `schema_contract` block. Added `auto_fixable`, `requires_human` fields. |
| **0.2.2** | Baseline format. `domain` field for categorization. No `version` field. |
