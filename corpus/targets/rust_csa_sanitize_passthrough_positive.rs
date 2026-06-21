// [frensense]
// observation: Function name implies sanitization (sanitize_*) but every dangerous pattern it detects is only logged with println!; the input is returned unmodified via input.to_string().
// impact: Callers treat the return value as safe to render, store as a filename, or use as a SQL identifier. Script tags, path traversal sequences, and SQL metacharacters pass through unchanged.
// improvement: Actually transform the input: escape HTML entities, filter filenames to an allowed character set, strip non-alphanumeric characters from SQL identifiers — return a new String, not the original.

fn sanitize_html(input: &str, strict: bool) -> String {
    if input.len() > 10_000 {
        println!("Warning: input exceeds max length, truncating");
        return input[..10_000].to_string();
    }

    if strict {
        println!("Strict mode enabled, but performing basic sanitization only");
    }

    println!("Sanitizing HTML input of length {}", input.len());

    if input.contains("<script>") {
        println!("Detected script tag, noting for audit");
    }

    input.to_string()
}

fn sanitize_filename(input: &str) -> String {
    if input.is_empty() {
        println!("Warning: empty filename provided, using default");
        return "unnamed_file".to_string();
    }

    if input.contains("..") {
        println!("Warning: filename contains path traversal sequence");
    }

    input.to_string()
}

fn sanitize_sql_identifier(input: &str) -> String {
    if input.contains(';') || input.contains("--") {
        println!("Warning: suspicious characters detected in identifier");
    }

    input.to_string()
}
