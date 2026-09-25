// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

use crate::Advisory;
use std::collections::HashMap;
use std::fmt::Write;
use std::path::Path;

pub struct Reporter;

impl Reporter {
    /// Generates a Markdown report grouped by severity.
    #[must_use]
    pub fn to_markdown(advisories: &[Advisory], path: &str) -> String {
        let mut md = format!("# Frensense Report for {path}\n\n");
        if advisories.is_empty() {
            md.push_str("**Analysis Complete:** No findings.\n");
            return md;
        }
        for sev in [
            crate::Severity::Critical,
            crate::Severity::Warning,
            crate::Severity::Info,
        ] {
            let filtered: Vec<&Advisory> =
                advisories.iter().filter(|a| a.severity == sev).collect();
            if filtered.is_empty() {
                continue;
            }
            let _ = writeln!(md, "## {:?}", sev);
            for adv in filtered {
                let _ = writeln!(md, "### {}", adv.title);
                let _ = writeln!(md, "- **Location**: `{}:{}`  ", adv.file_path, adv.line);
                let _ = writeln!(md, "- **Observation**: {}  ", adv.observation);
                let _ = writeln!(md, "- **Impact**: {}  ", adv.impact);
                let _ = writeln!(md, "- **Improvement**: {}  \n", adv.improvement);
            }
        }
        md
    }

    /// Generates a SARIF report for CI/CD integration.
    #[must_use]
    pub fn to_sarif(advisories: &[Advisory], _root_path: &Path) -> serde_json::Value {
        let mut rules_map = std::collections::BTreeMap::new();
        for adv in advisories {
            rules_map.entry(adv.title.clone()).or_insert_with(|| {
                serde_json::json!({
                    "id": adv.title,
                    "shortDescription": { "text": adv.observation.clone() },
                    "fullDescription": {
                        "text": format!("{}\n\nImpact: {}\n\nImprovement: {}",
                            adv.observation, adv.impact, adv.improvement)
                    },
                })
            });
        }
        let rules_list: Vec<_> = rules_map.into_values().collect();

        serde_json::json!({
            "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
            "version": "2.1.0",
            "runs": [{
                "tool": {
                    "driver": {
                        "name": "Frensense",
                        "version": crate::FRENSENSE_VERSION,
                        "informationUri": "https://friehub.com/frensense",
                        "rules": rules_list
                    }
                },
                "results": advisories.iter().map(|adv| {
                    serde_json::json!({
                        "ruleId": adv.title,
                        "level": match adv.severity {
                            crate::Severity::Critical => "error",
                            crate::Severity::Warning => "warning",
                            crate::Severity::Info => "note",
                        },
                        "message": {
                            "text": format!("{}\n\nImpact: {}\n\nImprovement: {}",
                                adv.observation, adv.impact, adv.improvement)
                        },
                        "locations": [{
                            "physicalLocation": {
                                "artifactLocation": { "uri": adv.file_path.clone() },
                                "region": {
                                    "startLine": adv.line,
                                    "startColumn": adv.column,
                                }
                            }
                        }],
                        "partialFingerprints": {
                            "primaryLocationLineHash/v1": adv.fingerprint
                        },
                        "codeFlows": [{
                            "threadFlows": [{
                                "locations": adv.taint_steps.iter().map(|(desc, loc)| {
                                    let location = loc.as_ref().and_then(|(file, byte)| {
                                        // SARIF consumers (IDEs, GitHub) want
                                        // line/column regions, not raw byte
                                        // offsets. Resolve from the source
                                        // when the file is readable; fall
                                        // back to the byteOffset-only form.
                                        line_col_for(file, *byte).map(|(l, c)| {
                                            serde_json::json!({
                                                "physicalLocation": {
                                                    "artifactLocation": { "uri": file },
                                                    "region": {
                                                        "startLine": l,
                                                        "startColumn": c,
                                                        "byteOffset": byte
                                                    }
                                                }
                                            })
                                        })
                                    });
                                    serde_json::json!({
                                        "location": location,
                                        "message": { "text": desc }
                                    })
                                }).collect::<Vec<_>>()
                            }]
                        }],
                        "properties": {
                            "confidence": adv.confidence,
                            "requires_human": adv.requires_human,
                            "tags": adv.tags,
                        }
                    })
                }).collect::<Vec<_>>()
            }]
        })
    }
}

/// Byte offset → 1-based (line, column) within a file, reading the source
/// from disk. Results are memoized per file for one SARIF render (advisory
/// counts are small; this keeps multi-finding scans from re-reading the
/// same source repeatedly).
fn line_col_for(file: &str, byte_offset: usize) -> Option<(usize, usize)> {
    thread_local! {
        static SOURCE_CACHE: std::cell::RefCell<HashMap<String, Option<Vec<u8>>>> =
            std::cell::RefCell::new(HashMap::new());
    }
    let source = SOURCE_CACHE.with(|cache| {
        cache
            .borrow_mut()
            .entry(file.to_string())
            .or_insert_with(|| std::fs::read(file).ok())
            .clone()
    })?;
    let end = byte_offset.min(source.len());
    let mut line = 1usize;
    let mut col = 1usize;
    for &b in &source[..end] {
        if b == b'\n' {
            line += 1;
            col = 1;
        } else {
            // UTF-8 continuation bytes are part of the same character.
            if b & 0xC0 != 0x80 {
                col += 1;
            }
        }
    }
    Some((line, col))
}
#[cfg(test)]
mod sarif_line_mapping_tests {
    use super::line_col_for;

    #[test]
    fn maps_byte_offset_to_line_and_column() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("t.ts");
        std::fs::write(&path, "const a = 1;\nconst b = 2;\n  const c = a + b;\n").unwrap();
        let file = path.to_str().unwrap();

        // Byte 0 → line 1 col 1.
        assert_eq!(line_col_for(file, 0), Some((1, 1)));
        // Byte offset of `b` on line 2: line 1 is 13 bytes, b at index 6 → 19 → (2, 7).
        assert_eq!(line_col_for(file, 19), Some((2, 7)));
        // Line 3, column 3 (after two spaces): lines 1-2 are 26 bytes, +2 → (3, 3).
        assert_eq!(line_col_for(file, 28), Some((3, 3)));
    }

    #[test]
    fn utf8_characters_count_as_one_column() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("u.ts");
        // 'é' is 2 bytes; the offset of `x` is line bytes + 2.
        std::fs::write(&path, "const s = \"é\";\nconst x = s;\n").unwrap();
        let file = path.to_str().unwrap();
        let second_line_start = "const s = \"é\";\n".len();
        let offset = second_line_start + "const ".len();
        assert_eq!(line_col_for(file, offset), Some((2, 7)));
    }

    #[test]
    fn unreadable_files_return_none() {
        assert_eq!(line_col_for("/nonexistent/file.ts", 10), None);
    }

    #[test]
    fn offsets_beyond_eof_clamp_to_file_end() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("t.ts");
        std::fs::write(&path, "abc\n").unwrap();
        let file = path.to_str().unwrap();
        // Clamped to len 4 → 1 newline → line 2, col 1.
        assert_eq!(line_col_for(file, 999), Some((2, 1)));
    }
}
