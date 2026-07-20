// SPDX-License-Identifier: MIT

use std::path::Path;

use crate::cfg::build_cfg;
use crate::cfg::def_use::compute_def_use;
use crate::corpus::source_sink::CorpusSourceSinkRegistry;

pub struct TaintConfidenceAdjuster;

impl TaintConfidenceAdjuster {
    pub fn adjust_confidence(
        source: &str,
        file_path: &Path,
        sink_line: u32,
        sink_content: &str,
        original_confidence: f32,
        registry: &CorpusSourceSinkRegistry,
    ) -> f32 {
        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let lang_name = crate::parser::ext_to_language(ext);
        if lang_name == "unknown" {
            return original_confidence;
        }

        let mut parser = tree_sitter::Parser::new();
        let lang = crate::parser::ParserRegistry::get_language_by_name(lang_name);
        let Ok(lang) = lang else {
            return original_confidence;
        };
        if parser.set_language(&lang).is_err() {
            return original_confidence;
        }
        let Some(tree) = parser.parse(source, None) else {
            return original_confidence;
        };
        let root = tree.root_node();

        let cfg = build_cfg(root, source, ext);
        let def_use = compute_def_use(&cfg, source);

        let var_name = extract_sink_var(sink_content);
        if var_name.is_empty() {
            return original_confidence;
        }

        let sink_byte = find_line_byte(source, sink_line);

        let candidates: Vec<_> = def_use
            .uses
            .iter()
            .enumerate()
            .filter(|(_, u)| {
                u.name == var_name
                    && sink_byte > 0
                    && u.start_byte <= sink_byte + 200
                    && u.end_byte >= sink_byte.saturating_sub(200)
            })
            .collect();

        if candidates.is_empty() {
            return original_confidence;
        }

        for (use_idx, use_) in &candidates {
            let defs_before: Vec<_> = def_use
                .definitions
                .iter()
                .filter(|d| {
                    d.block_id == use_.block_id
                        && d.name == var_name
                        && d.start_byte < use_.start_byte
                })
                .collect();

            let closest_def = defs_before.iter().max_by_key(|d| d.start_byte);

            let source_reaches = closest_def.is_some_and(|def| is_real_source(def, source, root, registry));

            if source_reaches {
                return original_confidence;
            }

            let inter_block_reaching = def_use.defs_reaching(*use_idx);
            for def in &inter_block_reaching {
                if def.block_id != use_.block_id
                    && def.name == var_name
                    && is_real_source(def, source, root, registry)
                {
                    return original_confidence;
                }
            }
        }

        (original_confidence * 0.35).max(0.15)
    }
}

fn extract_sink_var(content: &str) -> String {
    if let Some(paren_idx) = content.find('(') {
        let args = &content[paren_idx + 1..];
        if let Some(close_paren) = args.rfind(')') {
            let inner = &args[..close_paren];
            for part in inner.split(',') {
                let trimmed = part.trim();
                if !trimmed.is_empty()
                    && !trimmed.starts_with('"')
                    && !trimmed.starts_with('\'')
                    && !trimmed.starts_with('&')
                    && !trimmed.contains(' ')
                    && !trimmed.contains('.')
                {
                    return trimmed.to_string();
                }
            }
        }
    }
    String::new()
}

fn find_line_byte(source: &str, target_line: u32) -> usize {
    let mut line = 1u32;
    for (i, &b) in source.as_bytes().iter().enumerate() {
        if line == target_line {
            return i;
        }
        if b == b'\n' {
            line += 1;
        }
    }
    source.len()
}

fn is_real_source(
    def: &crate::cfg::def_use::Definition,
    source: &str,
    root: tree_sitter::Node,
    registry: &CorpusSourceSinkRegistry,
) -> bool {
    let mut start = def.start_byte.saturating_sub(5);
    while start > 0 && !source.is_char_boundary(start) {
        start -= 1;
    }
    let mut end = (def.end_byte + 40).min(source.len());
    while end < source.len() && !source.is_char_boundary(end) {
        end += 1;
    }
    let context = &source[start..end];
    
    if crate::corpus::loader::TAINT_SOURCE_PATTERNS.iter().any(|&p| context.contains(p)) {
        return true;
    }

    if let Some(type_name) = resolve_declared_type(def, root, source) {
        return registry.is_source_type(&type_name);
    }

    false
}

fn resolve_declared_type(
    def: &crate::cfg::def_use::Definition,
    root: tree_sitter::Node,
    source: &str,
) -> Option<String> {
    let node = root.descendant_for_byte_range(def.start_byte, def.end_byte)?;

    let mut current = node;
    loop {
        match current.kind() {
            "variable_declarator" | "required_parameter" | "optional_parameter" | "parameter" | "identifier" => {
                let mut cursor = current.walk();
                for child in current.children(&mut cursor) {
                    match child.kind() {
                        "type_annotation" | "type_identifier" | "scoped_type_identifier" | "generic_type" => {
                            let ty = source[child.start_byte()..child.end_byte()].trim();
                            if !ty.is_empty() {
                                let clean = ty.trim_start_matches(':').trim();
                                return Some(clean.to_string());
                            }
                        }
                        _ => {}
                    }
                }
                if matches!(current.kind(), "variable_declarator" | "required_parameter" | "optional_parameter" | "parameter") {
                    break;
                }
            }
            "assignment_expression" | "assignment" | "expression_statement" => break,
            "function_definition" | "function_declaration" | "arrow_function" | "method_definition" | "function_item" => break,
            _ => {}
        }
        if let Some(parent) = current.parent() {
            current = parent;
        } else {
            break;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kill_reduces_confidence() {
        let source = r#"
fn reassign() {
    let data = get_password();
    data = "clean";
    store_in_db(data);
}
"#;
        let registry = CorpusSourceSinkRegistry::default();
        let confidence = TaintConfidenceAdjuster::adjust_confidence(
            source,
            Path::new("test.rs"),
            5,
            "store_in_db(data)",
            0.95,
            &registry,
        );
        assert!(
            confidence < 0.95,
            "reassignment should reduce confidence, got {confidence}"
        );
    }

    #[test]
    fn test_no_kill_preserves_confidence() {
        let source = r"
fn no_kill() {
    let data = req.query.id;
    store_in_db(data);
}
";
        let registry = CorpusSourceSinkRegistry::default();
        let confidence = TaintConfidenceAdjuster::adjust_confidence(
            source,
            Path::new("test.rs"),
            4,
            "store_in_db(data)",
            0.95,
            &registry,
        );
        assert!(
            confidence > 0.80,
            "direct flow should preserve high confidence, got {confidence}"
        );
    }

    #[test]
    fn test_hardcoded_constant_not_source() {
        // Here `input` is just a hardcoded string, but under the old logic
        // `matches_source_name` would see `input` and think it's a source.
        // It shouldn't be treated as a real source anymore.
        let source = r#"
fn test_constant() {
    let input = "hardcoded_constant";
    store_in_db(input);
}
"#;
        let registry = CorpusSourceSinkRegistry::default();
        let confidence = TaintConfidenceAdjuster::adjust_confidence(
            source,
            Path::new("test.rs"),
            4,
            "store_in_db(input)",
            0.95,
            &registry,
        );
        // Because it's not a real source, confidence should drop
        assert!(
            confidence < 0.95,
            "hardcoded constant in 'input' should reduce confidence, got {confidence}"
        );
    }

    #[test]
    fn test_unknown_language_returns_original() {
        let registry = CorpusSourceSinkRegistry::default();
        let confidence =
            TaintConfidenceAdjuster::adjust_confidence("", Path::new("test.abc"), 1, "", 0.90, &registry);
        assert!(
            (confidence - 0.90).abs() < f32::EPSILON,
            "unknown language should return original"
        );
    }
}
