// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata, Severity};
use std::borrow::Cow;
use std::sync::OnceLock;
use tree_sitter::Node;

pub struct TautologicalAssert;

static METADATA: OnceLock<RuleMetadata> = OnceLock::new();

impl GenSenseRule for TautologicalAssert {
    fn metadata(&self) -> &RuleMetadata {
        METADATA.get_or_init(|| RuleMetadata {
            id: Cow::Borrowed("TS_TAUTOLOGICAL_ASSERT"),
            name: Cow::Borrowed("Tautological Assertion"),
            severity: Severity::Warning,
            observation: Cow::Borrowed("A tautological assertion was detected (e.g., expect(x).toBe(x))."),
            impact: Cow::Borrowed("Assertions that cannot fail do not contribute to correctness and may be AI-generated artifacts."),
            improvement: Cow::Borrowed("Replace with a meaningful assertion or remove the redundant check."),
            tags: vec![Cow::Borrowed("ai-risk"), Cow::Borrowed("correctness"), Cow::Borrowed("typescript")],
            category: Cow::Borrowed("Logic"),
            confidence: 0.85,
            precision: crate::Precision::High,
        })
    }

    fn applies_to(&self, ext: &str) -> bool {
        crate::parser::ParserRegistry::ext_matches(ext, &["ts", "tsx"])
    }

    fn query(&self) -> Option<&str> {
        Some("(call_expression) @call")
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let Some(func) = node.child_by_field_name("function") else {
            return Vec::new();
        };

        if func.kind() != "member_expression" {
            return Vec::new();
        }

        // Check that the outer call is a matcher like .toBe(), .toEqual(), etc.
        let Some(prop) = func.child_by_field_name("property") else {
            return Vec::new();
        };
        let matcher = &context.source_code[prop.start_byte()..prop.end_byte()];

        // The object of the member_expression should be expect(...)
        let Some(object) = func.child_by_field_name("object") else {
            return Vec::new();
        };
        if object.kind() != "call_expression" {
            return Vec::new();
        }
        let callee = match object.child_by_field_name("function") {
            Some(f) => &context.source_code[f.start_byte()..f.end_byte()],
            None => return Vec::new(),
        };
        if callee != "expect" {
            return Vec::new();
        }

        // Get the argument text from the inner expect(...) call.
        // Use the arguments node's text minus the parens to avoid relying on
        // which children tree-sitter exposes for keyword literals (true/false/null).
        let inner_arg = object.child_by_field_name("arguments").and_then(|a| {
            let text = &context.source_code[a.start_byte()..a.end_byte()];
            // Strip surrounding parens: "(true)" → "true"
            let stripped = text.trim();
            let inner = stripped
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .map_or(stripped, str::trim);
            if inner.is_empty() { None } else { Some(inner) }
        });

        // Collect arguments of the outer matcher call — only the non-paren children
        let outer_arg = node.child_by_field_name("arguments").and_then(|a| {
            let text = &context.source_code[a.start_byte()..a.end_byte()];
            let stripped = text.trim();
            let inner = stripped
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .map_or(stripped, str::trim);
            if inner.is_empty() { None } else { Some(inner) }
        });

        let is_tautology = match matcher {
            "toBe" | "toEqual" | "toStrictEqual" | "toContain" | "toHaveBeenCalledWith" => {
                inner_arg.zip(outer_arg).is_some_and(|(a, b)| a == b)
            }
            "toBeNull" => inner_arg == Some("null"),
            "toBeUndefined" | "toBeDefined" => inner_arg == Some("undefined"),
            "toBeTruthy" => inner_arg == Some("true"),
            "toBeFalsy" => inner_arg == Some("false"),
            _ => false,
        };

        if is_tautology {
            vec![self.new_advisory(
                &node,
                context,
                format!(
                    "Tautological assertion: expect({}).{}() — both sides are identical or the value is already the checked literal.",
                    inner_arg.unwrap_or("?"),
                    matcher,
                ),
            )]
        } else {
            Vec::new()
        }
    }
}
