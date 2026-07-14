use crate::data_flow::alias::AliasTracker;
use std::collections::{HashMap, HashSet};
use tree_sitter::Node;

/// Extracts (source_call, sink_call) data flow edges from a function.
pub fn extract_data_flows(func_node: Node<'_>, source: &str) -> HashSet<(String, String)> {
    let mut flows = HashSet::new();
    let mut taints: HashMap<String, HashSet<String>> = HashMap::new();
    let mut alias_tracker = AliasTracker::new();

    // Fast iterative pre-order traversal
    let mut stack = vec![func_node];
    let mut visited_count = 0;

    while let Some(node) = stack.pop() {
        visited_count += 1;
        if visited_count > 1_000 {
            break;
        }
        match node.kind() {
            "variable_declarator" | "assignment_expression" => {
                let (left, right) = if node.kind() == "variable_declarator" {
                    (
                        node.child_by_field_name("name"),
                        node.child_by_field_name("value"),
                    )
                } else {
                    (
                        node.child_by_field_name("left"),
                        node.child_by_field_name("right"),
                    )
                };

                if let (Some(l), Some(r)) = (left, right) {
                    let l_name = extract_var_name(l, source);
                    let r_name = extract_var_name(r, source);
                    if !l_name.is_empty() && !r_name.is_empty() {
                        alias_tracker.record_alias(&l_name, &r_name);
                    }
                    let r_taints = fast_evaluate_taint(r, source, &taints, &alias_tracker);
                    if !l_name.is_empty() && !r_taints.is_empty() {
                        taints.entry(l_name).or_default().extend(r_taints);
                    }
                }
            }
            "call_expression" => {
                let callee = node
                    .child_by_field_name("function")
                    .or_else(|| node.child_by_field_name("callee"));
                let args = node.child_by_field_name("arguments");

                if let (Some(c), Some(a)) = (callee, args) {
                    let sink_name = extract_callee_name(c, source);
                    if !sink_name.is_empty() {
                        let arg_taints = fast_evaluate_taint(a, source, &taints, &alias_tracker);
                        for t in arg_taints {
                            if t != sink_name {
                                flows.insert((t.clone(), sink_name.clone()));
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        let mut children = Vec::new();
        let mut child_cursor = node.walk();
        for child in node.children(&mut child_cursor) {
            children.push(child);
        }
        for child in children.into_iter().rev() {
            stack.push(child);
        }
    }

    flows
}

fn fast_evaluate_taint(
    root: Node<'_>,
    source: &str,
    env: &HashMap<String, HashSet<String>>,
    alias_tracker: &AliasTracker,
) -> HashSet<String> {
    let mut result = HashSet::new();
    let mut stack = vec![root];
    let mut visited_count = 0;

    // Avoid re-visiting nodes
    // Tree-sitter nodes don't implement Hash/Eq directly in a way we can easily use a HashSet for visited,
    // but a pre-order traversal of a tree without back-edges doesn't loop anyway.

    while let Some(node) = stack.pop() {
        visited_count += 1;
        if visited_count > 100 {
            break;
        }
        if node.kind() == "call_expression" {
            let callee = node
                .child_by_field_name("function")
                .or_else(|| node.child_by_field_name("callee"));
            if let Some(c) = callee {
                let name = extract_callee_name(c, source);
                if !name.is_empty() {
                    result.insert(name.clone());
                }
                if c.kind() == "member_expression" {
                    if let Some(obj) = c.child_by_field_name("object") {
                        stack.push(obj);
                    }
                }
            }
        } else if node.kind() == "await_expression" {
            if let Some(arg) = node.child(1) {
                stack.push(arg);
            }
        }

        if matches!(
            node.kind(),
            "identifier" | "field_identifier" | "property_identifier"
        ) {
            let name = source[node.start_byte()..node.end_byte()].to_string();
            if let Some(ts) = env.get(&name) {
                result.extend(ts.iter().cloned());
            } else {
                let mut found = false;
                for alias in alias_tracker.get_aliases(&name) {
                    if let Some(ts) = env.get(alias) {
                        result.extend(ts.iter().cloned());
                        found = true;
                    }
                }
                if !found {
                    result.insert(name);
                }
            }
        }

        // Push children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            stack.push(child);
        }
    }

    result
}

fn extract_var_name(node: Node<'_>, source: &str) -> String {
    match node.kind() {
        "identifier" | "field_identifier" => source[node.start_byte()..node.end_byte()].to_string(),
        "member_expression" => {
            if let Some(prop) = node.child_by_field_name("property") {
                return source[prop.start_byte()..prop.end_byte()].to_string();
            }
            String::new()
        }
        _ => String::new(),
    }
}

fn extract_callee_name(node: Node<'_>, source: &str) -> String {
    match node.kind() {
        "identifier" | "field_identifier" => source[node.start_byte()..node.end_byte()].to_string(),
        "member_expression" => {
            if let Some(field) = node
                .child_by_field_name("property")
                .or_else(|| node.child_by_field_name("field"))
            {
                return source[field.start_byte()..field.end_byte()].to_string();
            }
            if let Some(last) = node.child(node.child_count() - 1) {
                return source[last.start_byte()..last.end_byte()].to_string();
            }
            String::new()
        }
        "scoped_identifier" => {
            let text = source[node.start_byte()..node.end_byte()].to_string();
            text.rsplit("::").next().unwrap_or(&text).to_string()
        }
        _ => String::new(),
    }
}
