// SAFE: Uses regex-based replacement for all sanitization operations
use regex::Regex;

fn sanitize_html(input: &str) -> String {
    let re = Regex::new(r"[<>&\"']").unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        match &caps[0] {
            "&" => "&amp;".to_string(),
            "<" => "&lt;".to_string(),
            ">" => "&gt;".to_string(),
            "\"" => "&quot;".to_string(),
            "'" => "&#x27;".to_string(),
            _ => unreachable!(),
        }
    }).to_string()
}

fn sanitize_filename(input: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9._-]").unwrap();
    let cleaned = re.replace_all(input, "_").to_string();
    let trimmed = cleaned.trim_start_matches('.');
    if trimmed.is_empty() {
        "unnamed_file".to_string()
    } else {
        trimmed.chars().take(255).collect()
    }
}

fn sanitize_sql_identifier(input: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9_]").unwrap();
    re.replace_all(input, "").to_string()
}
