use crate::{Advisory, GenSenseContext, GenSenseRule};
use std::collections::HashSet;
use tree_sitter::Node;

pub struct RedundantComment;

impl GenSenseRule for RedundantComment {
    fn id(&self) -> &str {
        "RUST_REDUNDANT_COMMENT"
    }
    fn description(&self) -> &str {
        "Documentation that merely restates the identifier name."
    }
    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }
    fn query(&self) -> Option<&str> {
        Some("(function_item) @func")
    }

    fn check<'a>(&self, node: Node<'a>, context: & GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        if let Some(name_node) = node.child_by_field_name("name") {
            let name =
                &context.source_code[name_node.start_byte()..name_node.end_byte()].to_lowercase();
            let mut idents = HashSet::new();
            for part in name.split('_') {
                idents.insert(part.to_string());
            }

            let mut prev = node.prev_sibling();
            let mut doc_words = HashSet::new();
            let stop_words: HashSet<&str> = [
                "the", "a", "an", "this", "that", "is", "are", "was", "be", "to", "for", "with",
                "by", "of", "and", "or", "but", "in", "on", "at",
            ]
            .iter()
            .cloned()
            .collect();

            // "Why" keywords that suggest value-add documentation
            let value_keywords: HashSet<&str> = [
                "because",
                "since",
                "ensures",
                "guarantees",
                "handles",
                "prevents",
                "avoids",
                "logic",
                "algorithm",
            ]
            .iter()
            .cloned()
            .collect();
            let mut has_value_word = false;

            while let Some(sibling) = prev {
                if sibling.kind() == "line_comment" {
                    let comment = &context.source_code[sibling.start_byte()..sibling.end_byte()]
                        .to_lowercase();
                    if comment.starts_with("///") {
                        let text = comment.trim_start_matches("///").trim();
                        for word in text.split_whitespace() {
                            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());
                            if clean_word.len() > 2 && !stop_words.contains(clean_word) {
                                doc_words.insert(clean_word.to_string());
                                if value_keywords.contains(clean_word) {
                                    has_value_word = true;
                                }
                            }
                        }
                    }
                } else {
                    break;
                }
                prev = sibling.prev_sibling();
            }

            if !doc_words.is_empty() && !has_value_word {
                let intersection = doc_words.intersection(&idents).count();
                let overlap = intersection as f64 / doc_words.len() as f64;
                // Higher threshold (0.8) and requires at least 3 doc words to avoid noise on short comments
                if overlap > 0.8 && doc_words.len() >= 3 {
                    advisories.push(self.new_advisory(
                        &node,
                        "Redundant Comment Warning: docstring merely restates the identifier.".to_string(),
                        "Documentation that doesn't add value beyond the identifier name is considered architectural noise.".to_string(),
                        "Add semantic context (the 'why') or remove the redundant comment.".to_string(),
                    ));
                }
            }
        }
        advisories
    }
}
