use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct PlaceholderPanic;

impl GenSenseRule for PlaceholderPanic {
    fn metadata(&self) -> &crate::RuleMetadata {
        static META: std::sync::LazyLock<crate::RuleMetadata> = std::sync::LazyLock::new(|| {
            crate::RuleMetadata {
                id: "AI_PLACEHOLDER_PANIC".into(),
                name: "Placeholder Panic".into(),
                severity: crate::Severity::Critical,
                observation: "A 'todo!' or 'unimplemented!' placeholder macro was detected.".into(),
                impact: "Placeholders left in production paths lead to runtime panics and represent incomplete logic.".into(),
                improvement: "Replace with a functional implementation or handle the error path gracefully (e.g., return Result::Err).".into(),
                tags: vec!["ai-risk".into(), "security".into(), "reliability".into()],
                category: "Logic".into(),
            }
        });
        &META
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }

    fn query(&self) -> Option<&str> {
        Some("(macro_invocation) @macro")
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let Some(macro_name_node) = node.child(0) else {
            return advisories;
        };
        let macro_name =
            &context.source_code[macro_name_node.start_byte()..macro_name_node.end_byte()];

        if matches!(macro_name, "todo" | "unimplemented") {
            // Fire on ALL todo!/unimplemented! — bare placeholders are dangerous
            let advisory = self.new_advisory(
                &node,
                context,
                "Placeholder macro detected — this will panic unconditionally at runtime."
                    .to_string(),
            );
            advisories.push(self.with_confidence(advisory, 0.85));
        }

        advisories
    }
}
