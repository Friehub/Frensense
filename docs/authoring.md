# Rule Authoring & Schemas

GenSense rules are defined in YAML. To make authoring easier and less error-prone, we provide formal JSON schemas and LLM-powered generation tools.

## LLM-Powered Generation

If you use an LLM (like Claude 3.5, GPT-4o, or Gemini 1.5) to write rules, we recommend using our **GenSense Rule Expert** system prompt. This prompt provides the model with the necessary AST context and schema constraints.

- [View the GenSense Rule Expert Prompt](./prompts/rule-expert.md)

## JSON Schemas

You can link these schemas in your IDE (e.g., VS Code via the YAML extension) to get real-time validation and IntelliSense.

| Version | Description | Schema Link |
|---------|-------------|-------------|
| **v0.3.0** | **Latest.** Supports SRI, CSA project rules, and auto-remediation. | [v0.3.0 Schema](./schema/gensense-v3.schema.json) |
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

### v0.3.0 (Remediation & Governance)
- **Everything in v0.2.2**, plus:
- **Auto-Remediation**: `fix_pattern` and `fix_with` templates.
- **Import Injection**: `inject_import` with `{{root}}` resolution.
- **Project-Level Guards**: `must_have_guard` for cross-file CSA verification.
- **Symbol-Relative Identity**: Findings are anchored to functions/classes, not line numbers.
