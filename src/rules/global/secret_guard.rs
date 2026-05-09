// [LICENSE] Proprietary - Friehub (TaaS Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use crate::{Advisory, GenSenseContext, GenSenseRule};
use once_cell::sync::Lazy;
use regex::Regex;
use tree_sitter::Node;

pub struct SecretGuard;

static SECRET_RE: Lazy<Regex> = Lazy::new(|| {
    // Matches high-entropy patterns: Hex (32+ chars), Base64 (40+ chars)
    Regex::new(r"(?i)(0x[a-f0-9]{32,}|[a-z0-9+/]{40,}={0,2})")
        .expect("Internal Error: Failed to compile internal secret detection regex. This is a deployment blocker.")
});

impl GenSenseRule for SecretGuard {
    fn id(&self) -> &str {
        "SECRET_LEAK_DETECTION"
    }

    fn description(&self) -> &str {
        "Potential hardcoded secret or cryptographic key detected."
    }

    fn category(&self) -> &str {
        "Security"
    }
    fn severity(&self) -> crate::Severity {
        crate::Severity::Critical
    }

    fn query(&self) -> Option<&str> {
        None
    }

    fn applies_to(&self, _ext: &str) -> bool {
        true
    }

    fn check(&self, node: Node, context: &GenSenseContext) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        let kind = node.kind();
        if kind != "string_literal"
            && kind != "string"
            && kind != "string_content"
            && kind != "template_string"
        {
            return advisories;
        }

        // Skip test and non-production files
        let path_str = context.file_path.to_string_lossy();
        let skip_patterns = [
            "test",
            "bench",
            "example",
            "mock",
            "fixture",
            "e2e",
            "snapshot",
            "__tests__",
            "vendor",
        ];
        if skip_patterns.iter().any(|p| path_str.contains(p)) {
            return advisories;
        }

        let code = &context.source_code[node.start_byte()..node.end_byte()];
        // Strip quotes safely if it looks like a quoted string
        let inner = if code.len() >= 2
            && (code.starts_with('"') || code.starts_with('\'') || code.starts_with('`'))
        {
            &code[1..code.len() - 1]
        } else {
            code
        };

        if SECRET_RE.is_match(inner) {
            let _pos = node.start_position();
            advisories.push(self.new_advisory(
                &node,
                "Potential plaintext secret or sensitive token leak detected.".to_string(),
                "Hardcoding secrets in source code is a critical security risk that can lead to unauthorized access and system compromise.".to_string(),
                "We strongly recommend moving this token to a secure environment variable or a TEE-audited secret manager.".to_string(),
            ));
        }

        advisories
    }
}
