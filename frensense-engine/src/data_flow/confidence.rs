// SPDX-License-Identifier: MIT

use std::path::Path;

use crate::cfg::build_cfg;
use crate::cfg::def_use::compute_def_use;

pub struct TaintConfidenceAdjuster;

impl TaintConfidenceAdjuster {
    pub fn adjust_confidence(
        source: &str,
        file_path: &Path,
        sink_line: u32,
        sink_content: &str,
        original_confidence: f32,
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

            let source_reaches = closest_def.is_some_and(|def| matches_source_name(def, source));

            if source_reaches {
                return original_confidence;
            }

            let inter_block_reaching = def_use.defs_reaching(*use_idx);
            for def in &inter_block_reaching {
                if def.block_id != use_.block_id
                    && def.name == var_name
                    && matches_source_name(def, source)
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

fn matches_source_name(def: &crate::cfg::def_use::Definition, source: &str) -> bool {
    let start = def.start_byte.saturating_sub(5);
    let end = (def.end_byte + 40).min(source.len());
    let context = &source[start..end];
    let lowered = context.to_lowercase();
    lowered.contains("password")
        || lowered.contains("secret")
        || lowered.contains("token")
        || lowered.contains("credential")
        || lowered.contains("taint")
        || lowered.contains("read_")
        || lowered.contains("get_")
        || lowered.contains("input")
        || lowered.contains("body")
        || lowered.contains("param")
        || lowered.contains("request")
        || lowered.contains("query")
        || lowered.contains("user")
        || lowered.contains("file")
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
        let confidence = TaintConfidenceAdjuster::adjust_confidence(
            source,
            Path::new("test.rs"),
            5,
            "store_in_db(data)",
            0.95,
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
    let data = get_password();
    store_in_db(data);
}
";
        let confidence = TaintConfidenceAdjuster::adjust_confidence(
            source,
            Path::new("test.rs"),
            4,
            "store_in_db(data)",
            0.95,
        );
        assert!(
            confidence > 0.80,
            "direct flow should preserve high confidence, got {confidence}"
        );
    }

    #[test]
    fn test_unknown_language_returns_original() {
        let confidence =
            TaintConfidenceAdjuster::adjust_confidence("", Path::new("test.abc"), 1, "", 0.90);
        assert!(
            (confidence - 0.90).abs() < f32::EPSILON,
            "unknown language should return original"
        );
    }
}
