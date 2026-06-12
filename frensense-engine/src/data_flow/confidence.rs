// SPDX-License-Identifier: MIT

use std::path::Path;

use crate::cfg::build_cfg;
use crate::cfg::def_use::compute_def_use;

#[derive(Debug, Clone)]
pub struct TaintConfidenceAdjuster;

impl TaintConfidenceAdjuster {
    pub fn filter(
        source: &str,
        _file_path: &Path,
        ext: &str,
        tainted_vars: &[(String, u32, u32)],
        sink_line: u32,
        sink_var_hint: &str,
        original_confidence: f32,
    ) -> f32 {
        let mut parser = tree_sitter::Parser::new();
        let lang = crate::parser::ParserRegistry::get_language_by_name(
            match ext {
                "rs" => "rust",
                "ts" | "tsx" | "js" | "jsx" => "typescript",
                _ => return original_confidence,
            },
        );
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

        let sink_byte = find_byte_at_line(source, sink_line);

        let candidates: Vec<_> = def_use
            .uses
            .iter()
            .enumerate()
            .filter(|(_, u)| {
                u.name == sink_var_hint
                    || (sink_byte > 0
                        && u.start_byte <= sink_byte
                        && u.end_byte >= sink_byte)
            })
            .collect();

        if candidates.is_empty() {
            return original_confidence;
        }

        for (use_idx, use_) in &candidates {
            let defs_in_block: Vec<_> = def_use
                .definitions
                .iter()
                .enumerate()
                .filter(|(_, d)| d.block_id == use_.block_id && d.start_byte < use_.start_byte)
                .collect();

            let closest_def_before = defs_in_block
                .iter()
                .max_by_key(|(_, d)| d.start_byte)
                .map(|(_, d)| d);

            let mut source_reaches = false;
            if let Some(closest_def) = closest_def_before {
                for (t_var, t_line, _t_col) in tainted_vars {
                    if closest_def.name == *t_var
                        && byte_at_line(source, closest_def.start_byte) as u32 == *t_line
                    {
                        source_reaches = true;
                        break;
                    }
                }
            }

            if source_reaches {
                return original_confidence;
            }

            let inter_block_reaching = def_use.defs_reaching(*use_idx);
            for def in &inter_block_reaching {
                if def.block_id != use_.block_id {
                    for (t_var, t_line, _t_col) in tainted_vars {
                        if def.name == *t_var
                            && byte_at_line(source, def.start_byte) as u32 == *t_line
                        {
                            source_reaches = true;
                            break;
                        }
                    }
                }
            }
            if source_reaches {
                return original_confidence;
            }
        }

        (original_confidence * 0.35).max(0.15)
    }
}

fn find_byte_at_line(source: &str, target_line: u32) -> usize {
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

fn byte_at_line(source: &str, byte: usize) -> usize {
    source[..byte].matches('\n').count() + 1
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
        let confidence = TaintConfidenceAdjuster::filter(
            source,
            Path::new("test.rs"),
            "rs",
            &[("data".to_string(), 3, 9)],
            5,
            "data",
            0.95,
        );
        assert!(
            confidence < 0.95,
            "reassignment should reduce confidence, got {confidence}"
        );
    }

    #[test]
    fn test_no_kill_preserves_confidence() {
        let source = r#"
fn no_kill() {
    let data = get_password();
    store_in_db(data);
}
"#;
        let confidence = TaintConfidenceAdjuster::filter(
            source,
            Path::new("test.rs"),
            "rs",
            &[("data".to_string(), 3, 9)],
            4,
            "data",
            0.95,
        );
        assert!(
            confidence > 0.80,
            "direct flow should preserve high confidence, got {confidence}"
        );
    }
}
