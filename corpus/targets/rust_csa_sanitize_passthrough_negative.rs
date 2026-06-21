use std::collections::HashMap;

struct Sanitizer {
    html_entities: HashMap<char, &'static str>,
}

impl Sanitizer {
    fn new() -> Self {
        let mut html_entities = HashMap::new();
        html_entities.insert('&', "&amp;");
        html_entities.insert('<', "&lt;");
        html_entities.insert('>', "&gt;");
        html_entities.insert('"', "&quot;");
        html_entities.insert('\'', "&#x27;");
        Sanitizer { html_entities }
    }

    fn sanitize_html(&self, input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        for c in input.chars() {
            match self.html_entities.get(&c) {
                Some(escaped) => out.push_str(escaped),
                None => out.push(c),
            }
        }
        out
    }

    fn sanitize_filename(&self, input: &str) -> String {
        let cleaned: String = input
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' { c } else { '_' })
            .collect();
        let trimmed = cleaned.trim_start_matches('.');
        if trimmed.is_empty() {
            "unnamed_file".to_string()
        } else {
            trimmed.chars().take(255).collect()
        }
    }

    fn sanitize_sql_identifier(&self, input: &str) -> String {
        input
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect()
    }
}
