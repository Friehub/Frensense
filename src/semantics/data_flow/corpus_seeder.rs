// SPDX-License-Identifier: MIT

//! Corpus-driven taint seeding.
//!
//! Instead of regex rules, this module seeds taint based on corpus patterns.
//! When the corpus finds a function that matches a known bug pattern,
//! this seeder identifies which parameters are likely tainted based on
//! the parameter's type annotation, using the corpus-learned source registry.

use super::TaintRegistry;
use crate::semantics::data_flow::TaintOrigin;
use frensense_engine::corpus::source_sink::{CorpusSourceSinkRegistry, extract_param_info};
use tree_sitter::Node;

/// Seed taint for a function based on corpus pattern matching.
///
/// Uses the corpus-learned source type registry: taints parameters whose
/// type annotations match types found in positive corpus examples.
pub fn seed_from_corpus_match(
    fn_node: Node,
    source: &str,
    registry: &mut TaintRegistry,
    source_sink: &CorpusSourceSinkRegistry,
) {
    let params_node = match fn_node
        .child_by_field_name("parameters")
        .or_else(|| fn_node.child_by_field_name("formal_parameters"))
    {
        Some(p) => p,
        None => return,
    };

    let mut cursor = params_node.walk();
    for param in params_node.children(&mut cursor) {
        if matches!(param.kind(), "(" | ")" | "," | ";" | "self") {
            continue;
        }

        let (param_name, param_type) = extract_param_info(param, source);
        if param_name.is_empty() {
            continue;
        }

        let clean_type = param_type.trim_start_matches(':').trim();
        if source_sink.is_source_type(clean_type) {
            registry.taint(&param_name, TaintOrigin::UserInput);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_source_sink() -> CorpusSourceSinkRegistry {
        let mut reg = CorpusSourceSinkRegistry::default();
        reg.source_types.insert("Request".to_string(), 5);
        reg.source_types.insert("Json".to_string(), 3);
        reg
    }

    #[test]
    fn test_seed_framework_params() {
        let source = "function handler(req: Request, res: Response) { }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let mut registry = TaintRegistry::default();
        let source_sink = test_source_sink();
        seed_from_corpus_match(fn_node, source, &mut registry, &source_sink);

        assert!(
            registry.is_tainted("req"),
            "req with Request type should be tainted"
        );
    }

    #[test]
    fn test_no_taint_without_framework_type() {
        let source = "function process(input: string) { }";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let mut registry = TaintRegistry::default();
        let source_sink = test_source_sink();
        seed_from_corpus_match(fn_node, source, &mut registry, &source_sink);

        assert!(
            !registry.is_tainted("input"),
            "input without framework type should NOT be tainted"
        );
    }
}
