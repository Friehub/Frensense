// SPDX-License-Identifier: MIT

pub struct TaintRule {
    pub id: &'static str,
    pub source_re: &'static str,
    pub sink_re: &'static str,
    pub severity: crate::Severity,
    pub observation: &'static str,
    pub impact: &'static str,
    pub improvement: &'static str,
}

pub fn security_taint_rules() -> Vec<TaintRule> {
    vec![
        TaintRule {
            id: "TAINT_CREDENTIAL_TO_DB",
            source_re: "password|secret|token|credential|key|api_key",
            sink_re: "insert|update|create|upsert|db\\.|database\\.|query|execute",
            severity: crate::Severity::Critical,
            observation: "Plaintext credential may reach a database write.",
            impact: "Credentials stored without hashing are exposed in database dumps and logs.",
            improvement: "Hash with bcrypt/argon2 before persistence. Use parameterized queries.",
        },
        TaintRule {
            id: "TAINT_INPUT_TO_EXEC",
            source_re: "input|body|param|query|request|user|header|cookie",
            sink_re: "exec|system|shell|command|spawn|eval|popen",
            severity: crate::Severity::Critical,
            observation: "Untrusted input may reach a command execution sink.",
            impact: "Command injection allows arbitrary code execution on the host.",
            improvement: "Use parameterized APIs instead of string interpolation. Validate against allowlist.",
        },
        TaintRule {
            id: "TAINT_CREDENTIAL_TO_LOG",
            source_re: "password|secret|token|api_key|credential|private_key",
            sink_re: "log|console|print|write|debug|trace|info|warn|error",
            severity: crate::Severity::Warning,
            observation: "Credential may reach a logging sink.",
            impact: "Secrets in logs are captured by monitoring systems and stored indefinitely.",
            improvement: "Redact credentials before logging. Use a secrets scanner in CI.",
        },
        TaintRule {
            id: "TAINT_INPUT_TO_FS",
            source_re: "input|body|param|query|request|file_name|path",
            sink_re: "write|create|open|read_file|write_file|mkdir|remove",
            severity: crate::Severity::Warning,
            observation: "Untrusted input may reach a filesystem operation.",
            impact: "Path traversal allows reading/writing arbitrary files.",
            improvement: "Sanitize paths with a whitelist. Use a chroot or sandboxed directory.",
        },
        TaintRule {
            id: "TAINT_INPUT_TO_HTTP",
            source_re: "input|body|param|query|request|user|header",
            sink_re: "fetch|http|request|get|post|put|curl|wget",
            severity: crate::Severity::Warning,
            observation: "Untrusted input may reach an outbound HTTP request.",
            impact: "SSRF allows the server to make requests to internal services.",
            improvement: "Validate URLs against an allowlist. Block internal IP ranges.",
        },
        TaintRule {
            id: "TAINT_CREDENTIAL_TO_HTTP",
            source_re: "password|secret|token|api_key|credential",
            sink_re: "fetch|http|request|get|post|put|url|endpoint",
            severity: crate::Severity::Critical,
            observation: "Credential may be sent over HTTP in cleartext.",
            impact: "Credentials transmitted without encryption are intercepted on the network.",
            improvement: "Use HTTPS. Never pass credentials in URL query parameters.",
        },
    ]
}
