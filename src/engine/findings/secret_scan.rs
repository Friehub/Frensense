use crate::Advisory;
use crate::Severity;
use crate::engine::findings::FindingContext;
use crate::engine::project::FileSnapshot;

/// Check for hardcoded secrets, weak password handling, and plaintext sensitive data.
pub fn find(snap: &FileSnapshot, _ctx: &FindingContext<'_>) -> Vec<Advisory> {
    let source = &snap.content;
    let fname = snap.path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let mut advisories = Vec::new();

    // Check for hardcoded secrets in config files
    if fname == "config.js" || fname.ends_with("-dao.js") || fname.ends_with("_dao.js") || fname == "all.js" {
        if let Some(line) = find_hardcoded_secret(source) {
            advisories.push(
                Advisory::bare("A6-HARDCODED_SECRET", Severity::Critical, snap.id, &snap.path,
                    "A cryptographic secret, API key, or password appears hardcoded as a string literal")
                    .with_line(line)
                    .with_impact("Hardcoded secrets are checked into version control and shared across all environments, enabling credential theft.")
                    .with_improvement("Store secrets in environment variables or a vault service and reference them at runtime.")
            );
        }
    }

    // Check for plaintext password storage in DAO files
    if fname.ends_with("user-dao.js") || fname.ends_with("user_dao.js") || fname.ends_with("profile-dao.js") || fname.ends_with("profile_dao.js") {
        let active = strip_comments(source);

        // Plaintext store: saving password without hash
        // Check that no active line BOTH assigns password AND calls bcrypt/hash
        let has_active_password_hash = active.lines().any(|l| {
            l.contains("password") && (l.contains("bcrypt.hash") || l.contains("hashSync") || l.contains("scrypt") || l.contains("argon"))
        });
        if active.contains("password") && !has_active_password_hash {
            // Find the specific storage line — prefer lines with trailing comment or comma
            // (object property context), fall back to any "password" line
            let line = find_active_line(source, "password //")
                .or_else(|| find_active_line(source, "password,"))
                .or_else(|| find_active_line(source, "password"));
            if let Some(line) = line {
                if active.contains(".save(") || active.contains(".insert(") || active.contains(".update(") || active.contains("dao.")
                    || active.contains("password //") || active.contains("password,")
                {
                    advisories.push(
                        Advisory::bare("A2-PLAINTEXT_PW_STORE", Severity::Critical, snap.id, &snap.path,
                            "User password is stored without cryptographic hashing")
                            .with_line(line)
                            .with_impact("Storing plaintext passwords means a database breach leaks all user credentials in cleartext.")
                            .with_improvement("Hash passwords with bcrypt, scrypt, or argon2 before storing them.")
                    );
                }
            }
        }

        // Plaintext compare: comparing password with === instead of bcrypt.compare
        // Matches either lines containing both "password" and "===", or lines with
        // "===" inside functions named comparePassword or that compare parameters
        // whose names suggest password comparison (fromDB, fromUser, etc.)
        for line in find_active_lines(source, |l| {
            if l.contains("password") && l.contains("===") {
                return true;
            }
            if l.contains("===") && (l.contains("fromDB") || l.contains("fromUser")
                || l.contains("comparePassword") || l.contains("compare_password"))
            {
                return true;
            }
            false
        }) {
            advisories.push(
                Advisory::bare("A2-PLAINTEXT_PW_COMPARE", Severity::Critical, snap.id, &snap.path,
                    "User password is compared using === instead of a constant-time hash comparison")
                    .with_line(line)
                    .with_impact("Plaintext password comparison leaks the password length and timing side-channels, and implies passwords are stored without hashing.")
                    .with_improvement("Use bcrypt.compare() for password verification.")
            );
        }
    }

    // Check for SSN stored without encryption in profile DAO
    if fname.ends_with("profile-dao.js") || fname.ends_with("profile_dao.js") || fname.contains("profile") {
        let active = strip_comments(source);
        if !active.contains("encrypt") && !active.contains("cipher") && !active.contains("aes") {
            let ssn_lines: Vec<u32> = source.lines().enumerate()
                .filter(|(_, l)| {
                    let t = l.trim();
                    (t.contains(".ssn") || t.contains("\"ssn\"") || t.contains("'ssn'"))
                        && !t.starts_with("//") && !t.starts_with("*")
                })
                .map(|(i, _)| (i + 1) as u32)
                .collect();
            for line in ssn_lines {
                advisories.push(
                    Advisory::bare("A6-SSN_PLAINTEXT", Severity::Critical, snap.id, &snap.path,
                        "Social Security Number (SSN) or tax ID is stored or transmitted without encryption")
                        .with_line(line)
                        .with_impact("Exposing plaintext SSNs violates regulatory compliance (PCI-DSS, HIPAA, GDPR) and enables identity theft.")
                        .with_improvement("Encrypt SSNs at rest using AES-256 and in transit using TLS. Mask or truncate SSNs in logs and responses.")
                );
            }
        }
    }

    // Check for NoSQL injection via $where with template literal interpolation in DAO files
    if fname.ends_with("-dao.js") || fname.ends_with("_dao.js") {
        for line in find_active_lines(source, |l| {
            l.contains("$where") && l.contains("${")
        }) {
            advisories.push(
                Advisory::bare("A1-NOSQLI", Severity::Critical, snap.id, &snap.path,
                    "MongoDB $where clause contains template literal interpolation with user input, enabling NoSQL injection")
                    .with_line(line)
                    .with_impact("An attacker can inject arbitrary JavaScript into the $where expression, exfiltrating data or executing operations on the database server.")
                    .with_improvement("Avoid $where with string interpolation. Use typed query filters or validate/escape input before interpolation.")
            );
        }
    }

    // Check for weak random token generation (Math.random) in any file
    if source.contains("Math.random") && !source.contains("crypto") && !source.contains("uuid") && !source.contains("randomBytes") {
        for line in find_active_lines(source, |l| l.contains("Math.random")) {
            advisories.push(
                Advisory::bare("A2-WEAK_TOKEN", Severity::Warning, snap.id, &snap.path,
                    "Cryptographically weak random number generator (Math.random()) is used for security-sensitive tokens")
                    .with_line(line)
                    .with_impact("Math.random() is predictable and not suitable for session tokens, password reset tokens, or CSRF tokens.")
                    .with_improvement("Use crypto.randomBytes() or uuid v4 for security-sensitive random values.")
            );
        }
    }

    advisories
}

fn strip_comments(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let bytes = source.as_bytes();
    let mut i = 0;
    while i < source.len() {
        if i + 1 < source.len() && bytes[i] == b'/' && bytes[i + 1] == b'/' {
            while i < source.len() && bytes[i] != b'\n' { i += 1; }
            if i < source.len() && bytes[i] == b'\n' { result.push('\n'); i += 1; }
        } else if i + 1 < source.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
            while i + 1 < source.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                if bytes[i] == b'\n' { result.push('\n'); }
                i += 1;
            }
            if i + 1 < source.len() { i += 2; }
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }
    result
}

fn find_hardcoded_secret(source: &str) -> Option<u32> {
    let patterns = &[
        "cookieSecret", "sessionSecret", "jwtSecret", "apiKey", "api_key",
        "secret: '", "secret: \"", "secret : '", "secret : \"",
        "password: '", "password: \"",
    ];
    for p in patterns {
        if let Some(line) = find_active_line(source, p) {
            return Some(line);
        }
    }
    None
}

/// Find the first line in source that contains `pattern` and isn't inside a comment.
fn find_active_line(source: &str, pattern: &str) -> Option<u32> {
    find_active_lines(source, |l| l.contains(pattern)).into_iter().next()
}

/// Find all non-comment lines matching a predicate.
fn find_active_lines(source: &str, mut pred: impl FnMut(&str) -> bool) -> Vec<u32> {
    let mut in_block_comment = false;
    source.lines().enumerate().filter(|(_, line)| {
        let trimmed = line.trim();
        if trimmed.starts_with("/*") || in_block_comment {
            in_block_comment = !trimmed.contains("*/");
            return false;
        }
        if trimmed.starts_with("//") || trimmed.starts_with("*") {
            return false;
        }
        pred(line)
    }).map(|(i, _)| (i + 1) as u32).collect()
}