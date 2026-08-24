use crate::semantic_patterns::PatternFinding;
use crate::semantic_patterns::registry::SemanticPattern;
use tree_sitter::Node;

pub struct IdorMissingOwnershipCheck;

impl SemanticPattern for IdorMissingOwnershipCheck {
    fn id(&self) -> &str {
        "IDOR_MISSING_OWNERSHIP_CHECK"
    }

    fn description(&self) -> &str {
        "Function fetches a record using a user-controlled identifier and returns it without an ownership or authorization check"
    }

    fn severity(&self) -> &str {
        "High"
    }

    fn languages(&self) -> &[&str] {
        &["*"]
    }

    fn scan(&self, tree: Node, source: &str, file_path: &str) -> Vec<PatternFinding> {
        let ext = std::path::Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let mut findings = Vec::new();
        let mut cursor = tree.walk();
        let function_kinds: &[&str] = &[
            "function_declaration",
            "method_definition",
            "function_item",
            "function_definition",
            "arrow_function",
        ];

        loop {
            let node = cursor.node();
            if function_kinds.contains(&node.kind()) {
                let body = node.child_by_field_name("body");
                if let Some(body_node) = body {
                    let body_text = match body_node.utf8_text(source.as_bytes()) {
                        Ok(t) => t.to_lowercase(),
                        Err(_) => continue,
                    };
                    let fn_text = match node.utf8_text(source.as_bytes()) {
                        Ok(t) => t.to_lowercase(),
                        Err(_) => continue,
                    };

                    if self.has_idor_pattern(&body_text, &fn_text) {
                        let fn_name = node
                            .child_by_field_name("name")
                            .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                            .unwrap_or("<anonymous>")
                            .to_string();
                        let line = source[..node.start_byte()].lines().count() + 1;
                        let col = source[..node.start_byte()]
                            .rfind('\n')
                            .map_or(node.start_byte() + 1, |i| node.start_byte() - i);

                        findings.push(PatternFinding {
                            pattern_id: self.id().to_string(),
                            severity: self.severity().to_string(),
                            line,
                            column: col,
                            observation: format!("Function `{fn_name}` fetches a record using a user-controlled identifier and returns it without an ownership check"),
                            impact: "An attacker can enumerate or access records belonging to other users by changing the identifier parameter.".to_string(),
                            improvement: "Add an authorization check to verify the authenticated user owns or has permission to access the requested record before returning it.".to_string(),
                            confidence: 0.68,
                            tags: vec!["security".to_string(), "access-control".to_string(), "idor".to_string()],
                            enclosing_function: Some(fn_name),
                        });
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

impl IdorMissingOwnershipCheck {
    fn has_idor_pattern(&self, body_text: &str, fn_text: &str) -> bool {
        let has_tainted_param = body_text.contains("req.params")
            || body_text.contains("req.query")
            || body_text.contains("req.body")
            || body_text.contains("request.params")
            || body_text.contains("request.query")
            || body_text.contains("request.body")
            || body_text.contains("params.id")
            || body_text.contains("params.userid")
            || body_text.contains("query.id")
            || body_text.contains("query.userid");

        let has_db_fetch = body_text.contains(".find(")
            || body_text.contains(".findone(")
            || body_text.contains(".findfirst(")
            || body_text.contains(".findunique(")
            || body_text.contains(".findbyid(")
            || body_text.contains("findbypk(")
            || body_text.contains(".get(")
            || body_text.contains("query(`")
            || body_text.contains("raw(`")
            || body_text.contains(".where(");

        let has_response_send = body_text.contains("res.json(")
            || body_text.contains("res.send(")
            || body_text.contains("res.render(")
            || body_text.contains("response.json(")
            || body_text.contains("response.send(")
            || body_text.contains("return ok(")
            || body_text.contains("httpresponse::ok")
            || body_text.contains("c.json(")
            || body_text.contains("c.status(");

        let has_ownership_check = body_text.contains("userid")
            || body_text.contains("user_id")
            || body_text.contains("ownerid")
            || body_text.contains("owner_id")
            || body_text.contains("createdby")
            || body_text.contains("created_by")
            || body_text.contains("session.userid")
            || body_text.contains("session.user_id")
            || body_text.contains("req.user.id")
            || body_text.contains("req.userid")
            || body_text.contains("currentuser")
            || body_text.contains("current_user")
            || body_text.contains(".findfirst({")
                && (body_text.contains("userid")
                    || body_text.contains("user_id")
                    || body_text.contains("owner"))

                // Check for comparison patterns in the function body
                || (fn_text.contains("===") || fn_text.contains("=="))
                    && (fn_text.contains(".id") || fn_text.contains("userid") || fn_text.contains("user_id"))
                    && (fn_text.contains("req.") || fn_text.contains("session."))
                || body_text.contains("unauthorized")
                || body_text.contains("forbidden")
                || body_text.contains("not authorized")
                || body_text.contains("access denied")
                || body_text.contains("status(403")
                || body_text.contains("status(401")
                || fn_text.contains("403") && (fn_text.contains("user") || fn_text.contains("owner"))
                || fn_text.contains("401") && (fn_text.contains("user") || fn_text.contains("owner"));

        has_tainted_param && has_db_fetch && has_response_send && !has_ownership_check
    }
}
