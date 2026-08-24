use crate::semantic_patterns::PatternFinding;
use crate::semantic_patterns::helpers::collect_calls_in_scope;
use crate::semantic_patterns::registry::SemanticPattern;
use tree_sitter::Node;

pub struct CsrfMissingToken;

const STATE_MUTATING_ROUTE_VERBS: &[&str] = &[
    "app.post",
    "app.put",
    "app.delete",
    "app.patch",
    "router.post",
    "router.put",
    "router.delete",
    "router.patch",
    "server.post",
    "server.put",
    "server.delete",
    "server.patch",
];

impl SemanticPattern for CsrfMissingToken {
    fn id(&self) -> &str {
        "CSRF_MISSING_TOKEN"
    }

    fn description(&self) -> &str {
        "State-changing route handler does not include CSRF token validation"
    }

    fn severity(&self) -> &str {
        "High"
    }

    fn languages(&self) -> &[&str] {
        &["*"]
    }

    fn scan(&self, tree: Node, source: &str, _file_path: &str) -> Vec<PatternFinding> {
        let mut findings = Vec::new();
        let mut cursor = tree.walk();

        loop {
            let node = cursor.node();

            // Look for route registration calls: app.post("/path", handler)
            if node.kind() == "call_expression" {
                if let Some(func) = node.child_by_field_name("function") {
                    if let Ok(call_text) = func.utf8_text(source.as_bytes()) {
                        let is_state_mutating = STATE_MUTATING_ROUTE_VERBS
                            .iter()
                            .any(|v| call_text.contains(v));

                        if is_state_mutating {
                            let handler_node =
                                node.child_by_field_name("arguments").and_then(|args| {
                                    let mut c = args.walk();
                                    let mut last = None;
                                    if c.goto_first_child() {
                                        loop {
                                            last = Some(c.node());
                                            if !c.goto_next_sibling() {
                                                break;
                                            }
                                        }
                                    }
                                    last
                                });

                            if let Some(handler) = handler_node {
                                let has_csrf = self.handler_has_csrf(handler, source);

                                if !has_csrf {
                                    let line = source[..node.start_byte()].lines().count() + 1;
                                    let col = source[..node.start_byte()]
                                        .rfind('\n')
                                        .map_or(node.start_byte() + 1, |i| node.start_byte() - i);

                                    findings.push(PatternFinding {
                                        pattern_id: self.id().to_string(),
                                        severity: self.severity().to_string(),
                                        line,
                                        column: col,
                                        observation: format!("State-changing route `{call_text}` does not include CSRF token validation"),
                                        impact: "Without CSRF protection, an attacker can trick authenticated users into performing unintended state-changing actions.".to_string(),
                                        improvement: "Add CSRF token validation middleware (e.g., csurf, csrf-csrf, or SameSite=Strict cookie attribute) to this route.".to_string(),
                                        confidence: 0.72,
                                        tags: vec!["security".to_string(), "csrf".to_string(), "access-control".to_string()],
                                        enclosing_function: None,
                                    });
                                }
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
                    return findings;
                }
            }
        }
    }
}

impl CsrfMissingToken {
    fn handler_has_csrf(&self, handler: Node, source: &str) -> bool {
        let text = match handler.utf8_text(source.as_bytes()) {
            Ok(t) => t,
            Err(_) => return false,
        };
        let lower = text.to_lowercase();

        // Check for CSRF-related patterns in the handler
        if lower.contains("csrf")
            || lower.contains("xsrf")
            || lower.contains("csrf_token")
            || lower.contains("xsrf_token")
            || lower.contains("_csrf")
            || lower.contains("req.csrf")
            || lower.contains("req.session.csrf")
        {
            return true;
        }

        // Check for SameSite cookie attribute
        if lower.contains("samesite")
            || lower.contains("samesite=strict")
            || lower.contains("samesite=lax")
        {
            return true;
        }

        // Check for custom token validation logic
        let calls = collect_calls_in_scope(handler, source);
        for (callee, _) in &calls {
            let callee_lower = callee.to_lowercase();
            if callee_lower.contains("csrf")
                || callee_lower.contains("validate")
                || callee_lower.contains("verifytoken")
                || callee_lower == "verify"
            {
                return true;
            }
        }

        false
    }
}
