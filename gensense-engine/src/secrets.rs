// SPDX-License-Identifier: MIT

use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct SecretMatch {
    pub pattern_name: String,
    pub matched_text: String,
    pub line: usize,
    pub column: usize,
    pub confidence: f64,
    pub context: String,
    pub file_path: String,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug, Clone)]
pub struct SecretPattern {
    pub name: String,
    pub regex: Regex,
    pub confidence: f64,
    pub context_hint: Option<String>,
}

#[derive(Debug, Default)]
pub struct SecretScanner {
    patterns: Vec<SecretPattern>,
    entropy_threshold: f64,
}

impl SecretScanner {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_entropy_threshold(mut self, threshold: f64) -> Self {
        self.entropy_threshold = threshold;
        self
    }

    pub fn add_pattern(&mut self, name: &str, regex: &str, confidence: f64, context_hint: Option<&str>) {
        if let Ok(re) = Regex::new(regex) {
            self.patterns.push(SecretPattern {
                name: name.to_string(),
                regex: re,
                confidence,
                context_hint: context_hint.map(String::from),
            });
        }
    }

    pub fn add_default_patterns(&mut self) {
        self.add_pattern(
            "aws_access_key",
            r"(?i)AKIA[0-9A-Z]{16}",
            0.9,
            Some("AWS Access Key"),
        );
        self.add_pattern(
            "aws_secret_key",
            r"(?i)aws(.{0,20})?['][0-9a-zA-Z/+]{40}[']",
            0.95,
            Some("AWS Secret Key"),
        );
        self.add_pattern(
            "github_token",
            r"(?i)ghp_[0-9a-zA-Z]{36}",
            0.95,
            Some("GitHub Token"),
        );
        self.add_pattern(
            "generic_api_key",
            r"(?i)(api[_-]?key|apikey|secret|token|password).{0,5}['][0-9a-zA-Z_-]{16,64}[']",
            0.7,
            Some("Generic API Key / Secret"),
        );
        self.add_pattern(
            "jwt_token",
            r"eyJ[0-9a-zA-Z_-]+\.[0-9a-zA-Z_-]+\.[0-9a-zA-Z_-]+",
            0.85,
            Some("JWT Token"),
        );
        self.add_pattern(
            "private_key_header",
            r"-----BEGIN\s?(RSA|DSA|EC|OPENSSH|PGP)?\s?PRIVATE KEY-----",
            0.95,
            Some("Private Key"),
        );
        self.add_pattern(
            "connection_string",
            r"(?i)(mongodb|postgresql|mysql|redis)://[^\s]+",
            0.85,
            Some("Database Connection String"),
        );
        self.add_pattern(
            "slack_token",
            r"xox[baprs]-[0-9a-zA-Z-]{10,72}",
            0.9,
            Some("Slack Token"),
        );
        self.add_pattern(
            "google_api_key",
            r"AIza[0-9A-Za-z_-]{35}",
            0.85,
            Some("Google API Key"),
        );
    }

    pub fn scan_source(&self, source: &str, file_path: &Path) -> Vec<SecretMatch> {
        let mut results = Vec::new();
        let file_str = file_path.to_string_lossy().to_string();

        for pattern in &self.patterns {
            for m in pattern.regex.find_iter(source) {
                let line = source[..m.start()].matches('\n').count() + 1;
                let column = m.start() - source[..m.start()].rfind('\n').map_or(0, |i| i + 1) + 1;
                let context_start = m.start().saturating_sub(40);
                let context_end = (m.end() + 40).min(source.len());
                let entropy = shannon_entropy(m.as_str());

                let confidence = if entropy > self.entropy_threshold {
                    pattern.confidence
                } else {
                    pattern.confidence * 0.5
                };

                if confidence > 0.3 {
                    results.push(SecretMatch {
                        pattern_name: pattern.name.clone(),
                        matched_text: m.as_str().to_string(),
                        line,
                        column,
                        confidence,
                        context: source[context_start..context_end].to_string(),
                        file_path: file_str.clone(),
                        start_byte: m.start(),
                        end_byte: m.end(),
                    });
                }
            }
        }

        results
    }

    pub fn scan_tree(&self, root: Node, source: &str, file_path: &Path) -> Vec<SecretMatch> {
        let mut results = Vec::new();

        let mut cursor = root.walk();
        loop {
            let node = cursor.node();
            if node.kind() == "string" || node.kind() == "string_literal" || node.kind() == "template_string" {
                if let Ok(text) = node.utf8_text(source.as_bytes()) {
                    let line = node.start_position().row + 1;
                    let column = node.start_position().column + 1;
                    for pattern in &self.patterns {
                        for m in pattern.regex.find_iter(text) {
                            let entropy = shannon_entropy(m.as_str());
                            let confidence = if entropy > self.entropy_threshold {
                                pattern.confidence
                            } else {
                                pattern.confidence * 0.5
                            };
                            if confidence > 0.3 {
                                let context_start = node.start_byte().saturating_sub(20);
                                let context_end = (node.end_byte() + 20).min(source.len());
                                results.push(SecretMatch {
                                    pattern_name: pattern.name.clone(),
                                    matched_text: m.as_str().to_string(),
                                    line,
                                    column: column + m.start(),
                                    confidence,
                                    context: source[context_start..context_end].to_string(),
                                    file_path: file_path.to_string_lossy().to_string(),
                                    start_byte: node.start_byte() + m.start(),
                                    end_byte: node.start_byte() + m.end(),
                                });
                            }
                        }
                    }
                }
            }

            if cursor.goto_first_child() {
                continue;
            }
            loop {
                if cursor.goto_next_sibling() {
                    break;
                }
                if !cursor.goto_parent() {
                    return results;
                }
            }
        }
    }

    pub fn deduplicate(matches: &mut Vec<SecretMatch>) {
        let mut seen = std::collections::HashSet::new();
        matches.retain(|m| seen.insert((m.line, m.column, m.pattern_name.clone())));
    }

    pub fn group_by_pattern(matches: &[SecretMatch]) -> HashMap<String, Vec<&SecretMatch>> {
        let mut grouped: HashMap<String, Vec<&SecretMatch>> = HashMap::new();
        for m in matches {
            grouped.entry(m.pattern_name.clone()).or_default().push(m);
        }
        grouped
    }
}

fn shannon_entropy(s: &str) -> f64 {
    if s.is_empty() {
        return 0.0;
    }
    let mut freq = [0u64; 256];
    for &b in s.as_bytes() {
        freq[b as usize] = freq[b as usize].saturating_add(1);
    }
    let len = s.len() as f64;
    freq.iter().filter(|&&c| c > 0).fold(0.0f64, |acc, &count| {
        let p = count as f64 / len;
        acc - p * p.log2()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_aws_key() {
        let mut scanner = SecretScanner::new();
        scanner.add_default_patterns();
        let source = "let key = \"AKIAIOSFODNN7EXAMPLE\";";
        let results = scanner.scan_source(source, Path::new("test.rs"));
        let aws: Vec<_> = results.iter().filter(|r| r.pattern_name == "aws_access_key").collect();
        assert!(!aws.is_empty(), "should detect AWS access key");
    }

    #[test]
    fn test_scan_private_key() {
        let mut scanner = SecretScanner::new();
        scanner.add_default_patterns();
        let source = "-----BEGIN RSA PRIVATE KEY-----\nMIICXAIBAAKBgQ\n-----END RSA PRIVATE KEY-----";
        let results = scanner.scan_source(source, Path::new("test.rs"));
        let pk: Vec<_> = results.iter().filter(|r| r.pattern_name == "private_key_header").collect();
        assert!(!pk.is_empty(), "should detect private key header");
    }

    #[test]
    fn test_entropy_filtering() {
        let entropy = shannon_entropy("abc123");
        assert!(entropy > 0.0);
        let low_entropy = shannon_entropy("aaaa");
        assert!(low_entropy < entropy);
    }

    #[test]
    fn test_deduplicate() {
        let mut scanner = SecretScanner::new();
        scanner.add_default_patterns();
        let source = "\"AKIAIOSFODNN7EXAMPLE\" \"AKIAIOSFODNN7EXAMPLE\"";
        let mut results = scanner.scan_source(source, Path::new("test.rs"));
        let before = results.len();
        SecretScanner::deduplicate(&mut results);
        assert!(results.len() <= before);
    }

    #[test]
    fn test_group_by_pattern() {
        let mut scanner = SecretScanner::new();
        scanner.add_default_patterns();
        let source = "AKIAIOSFODNN7EXAMPLE and -----BEGIN RSA PRIVATE KEY-----";
        let results = scanner.scan_source(source, Path::new("test.rs"));
        let grouped = SecretScanner::group_by_pattern(&results);
        assert!(grouped.contains_key("aws_access_key") || grouped.contains_key("private_key_header"));
    }
}
