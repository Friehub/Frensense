use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct TsFloatingPromiseDetector;

impl GenSenseRule for TsFloatingPromiseDetector {
    fn metadata(&self) -> &crate::RuleMetadata {
        static META: std::sync::LazyLock<crate::RuleMetadata> = std::sync::LazyLock::new(|| {
            crate::RuleMetadata {
                id: "TS_FLOATING_PROMISE".into(),
                name: "Floating Promise".into(),
                severity: crate::Severity::Warning,
                observation: "A promise-returning call (fetch/prisma/etc) was detected without being awaited, returned, or assigned.".into(),
                impact: "Floating promises can lead to unhandled rejections and race conditions.".into(),
                improvement: "Ensure the promise is handled using 'await', 'return', or by assigning it to a variable.".into(),
                tags: vec!["ai-risk".into(), "reliability".into(), "async".into()],
                category: "Logic".into(),
                confidence: 0.85,
            }
        });
        &META
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "ts" || ext == "tsx"
    }

    fn query(&self) -> Option<&str> {
        Some("(call_expression) @call")
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let code = &context.source_code[node.start_byte()..node.end_byte()];

        let promise_sinks = ["fetch(", "prisma.", "axios.", "db.", "supabase."];
        if !promise_sinks.iter().any(|&sink| code.contains(sink)) {
            return advisories;
        }

        // Instead of string-matching, check the AST parent node kind
        let is_handled = node.parent().is_some_and(|p| {
            matches!(
                p.kind(),
                "await_expression"       // await fetch(...)
            | "return_statement"     // return fetch(...)
            | "variable_declarator"  // const x = fetch(...)
            | "assignment_expression"// x = fetch(...)
            | "lexical_declaration"  // let/const at declaration level
            | "arguments" // passed as arg to another call
            )
        });

        if !is_handled {
            advisories.push(self.new_advisory(
                &node,
                context,
                "Floating Promise: promise-returning call is not awaited, returned, or assigned."
                    .to_string(),
            ));
        }

        advisories
    }
}
