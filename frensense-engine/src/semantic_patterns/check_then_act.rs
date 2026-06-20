// SPDX-License-Identifier: MIT

use tree_sitter::Node;

use super::helpers::*;
use super::{PatternFinding, SemanticPattern};

/// Detects check-then-act (TOCTOU) patterns on database operations.
///
/// Pattern: A database read is followed by a conditional check on the result,
/// then a database write happens outside a transaction block.
///
/// This detects race conditions where concurrent requests can bypass the
/// check and cause overselling, double-spending, duplicate records, etc.
pub struct CheckThenAct;

impl SemanticPattern for CheckThenAct {
    fn id(&self) -> &str {
        "CHECK_THEN_ACT_TOCTOU"
    }

    fn description(&self) -> &str {
        "Detects read-then-check-then-write patterns without transactional atomicity"
    }

    fn severity(&self) -> &str {
        "Critical"
    }

    fn scan(&self, tree: Node, source: &str, file_path: &str) -> Vec<PatternFinding> {
        let mut findings = Vec::new();
        scan_function_bodies(tree, source, file_path, &mut findings);
        findings
    }
}

fn scan_function_bodies<'a>(
    node: Node<'a>,
    source: &'a str,
    file_path: &str,
    findings: &mut Vec<PatternFinding>,
) {
    // Look for function bodies (functions, arrow functions, methods)
    match node.kind() {
        "function_declaration"
        | "function"
        | "arrow_function"
        | "method_definition"
        | "function_item" => {
            if let Some(body) = find_function_body(node) {
                detect_toctou_in_scope(body, source, file_path, findings);
            }
        }
        _ => {}
    }

    // Recurse into children
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            scan_function_bodies(child, source, file_path, findings);
        }
    }
}

fn find_function_body(node: Node) -> Option<Node> {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            match child.kind() {
                "statement_block" | "block" | "body" => return Some(child),
                _ => {}
            }
        }
    }
    None
}

fn detect_toctou_in_scope<'a>(
    scope: Node<'a>,
    source: &'a str,
    file_path: &str,
    findings: &mut Vec<PatternFinding>,
) {
    // Collect all statements/declarations in the scope
    let statements = collect_statements(scope);

    // For each pair of (read, check, write), detect the TOCTOU pattern
    for (i, stmt) in statements.iter().enumerate() {
        // Step 1: Find database reads
        let db_reads = find_db_reads_in_node(*stmt, source);
        if db_reads.is_empty() {
            continue;
        }

        // Step 2: Look for conditional checks on the read result in subsequent statements
        for (read_var, read_node) in &db_reads {
            for j in (i + 1)..statements.len() {
                let check_stmt = statements[j];

                // Check if this is a conditional that references the read variable
                if !is_conditional_check(check_stmt) {
                    continue;
                }

                let cond_refs = extract_condition_refs(check_stmt, source);
                if !cond_refs.iter().any(|r| r == read_var) {
                    continue;
                }

                // Step 3: Look for database writes after the check, outside a transaction
                for k in (j + 1)..statements.len() {
                    let write_stmt = statements[k];

                    // Stop if we hit another function boundary
                    if matches!(
                        write_stmt.kind(),
                        "function_declaration"
                            | "function"
                            | "arrow_function"
                            | "method_definition"
                    ) {
                        break;
                    }

                    let db_writes = find_db_writes_in_node(write_stmt, source);
                    for (write_method, write_node) in &db_writes {
                        // Skip if the write is inside a $transaction block
                        if is_inside_transaction(*write_node, source) {
                            continue;
                        }

                        // Check if the write targets the same entity as the read
                        let read_entity = extract_entity_from_read(source, *read_node);
                        let write_entity = extract_entity_from_write(source, *write_node);

                        let same_entity = match (&read_entity, &write_entity) {
                            (Some(r), Some(w)) => r == w || w.contains(r),
                            _ => true, // If we can't determine entity, still flag it
                        };

                        if !same_entity {
                            continue;
                        }

                        let read_line = read_node.start_position().row + 1;
                        let write_line = write_node.start_position().row + 1;
                        let check_line = check_stmt.start_position().row + 1;

                        let read_text = node_text(*read_node, source);
                        let truncated_read = truncate_text(read_text, 60);

                        findings.push(PatternFinding {
                            pattern_id: "CHECK_THEN_ACT_TOCTOU".to_string(),
                            severity: "Critical".to_string(),
                            line: read_line,
                            column: read_node.start_position().column + 1,
                            observation: format!(
                                "Database read at line {read_line} is checked at line {check_line}, \
                                 then written at line {write_line} without transaction wrapping. \
                                 Read: `{truncated_read}`, Write method: `{write_method}`"
                            ),
                            impact: "Concurrent requests can bypass the check and cause \
                                     overselling, double-spending, or duplicate records."
                                .to_string(),
                            improvement: "Move the check inside a $transaction block, use \
                                         atomic updateMany with conditional WHERE, or use upsert."
                                .to_string(),
                            confidence: 0.80,
                            tags: vec![
                                "toctou".to_string(),
                                "race-condition".to_string(),
                                "concurrency".to_string(),
                            ],
                            enclosing_function: None,
                        });

                        // Only report once per read-check-write chain
                        break;
                    }
                }
            }
        }
    }
}

fn collect_statements<'a>(node: Node<'a>) -> Vec<Node<'a>> {
    let mut stmts = Vec::new();
    collect_statements_recursive(node, &mut stmts);
    stmts
}

fn collect_statements_recursive<'a>(node: Node<'a>, stmts: &mut Vec<Node<'a>>) {
    match node.kind() {
        "statement_block" | "block" => {
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if is_statement(child) {
                        stmts.push(child);
                    }
                }
            }
        }
        "if_statement" => {
            stmts.push(node);
            // Also collect from branches
            if let Some(consequence) = node.child_by_field_name("consequence") {
                collect_statements_recursive(consequence, stmts);
            }
            if let Some(alternative) = node.child_by_field_name("alternative") {
                collect_statements_recursive(alternative, stmts);
            }
        }
        "try_statement" => {
            stmts.push(node);
            if let Some(body) = node.child_by_field_name("body") {
                collect_statements_recursive(body, stmts);
            }
        }
        _ => {
            stmts.push(node);
        }
    }
}

fn is_statement(node: Node) -> bool {
    matches!(
        node.kind(),
        "expression_statement"
            | "return_statement"
            | "if_statement"
            | "for_statement"
            | "while_statement"
            | "do_statement"
            | "switch_statement"
            | "try_statement"
            | "variable_declaration"
            | "lexical_declaration"
            | "await_expression"
            | "throw_statement"
    )
}

fn find_db_reads_in_scope<'a>(node: Node<'a>, source: &'a str) -> Vec<(String, Node<'a>)> {
    let mut reads = Vec::new();
    find_db_reads_recursive(node, source, &mut reads);
    reads
}

fn find_db_reads_in_node<'a>(node: Node<'a>, source: &'a str) -> Vec<(String, Node<'a>)> {
    let mut reads = Vec::new();
    find_db_reads_recursive(node, source, &mut reads);
    reads
}

fn find_db_reads_recursive<'a>(node: Node<'a>, source: &'a str, reads: &mut Vec<(String, Node<'a>)>) {
    if let Some(method) = is_db_read(node, source) {
        // Extract the variable name this is assigned to
        if let Some(parent) = node.parent() {
            let var_name = extract_assigned_var(parent, source);
            if !var_name.is_empty() {
                reads.push((var_name, node));
            } else {
                // Try to infer from the call itself (e.g., `const x = await prisma.user.findUnique(...)`)
                let text = node_text(node, source);
                if let Some(entity) = extract_entity_from_read(source, node) {
                    reads.push((format!("__{entity}"), node));
                }
            }
        }
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            find_db_reads_recursive(child, source, reads);
        }
    }
}

fn find_db_writes_in_node<'a>(node: Node<'a>, source: &'a str) -> Vec<(String, Node<'a>)> {
    let mut writes = Vec::new();
    find_db_writes_recursive(node, source, &mut writes);
    writes
}

fn find_db_writes_recursive<'a>(node: Node<'a>, source: &'a str, writes: &mut Vec<(String, Node<'a>)>) {
    if let Some(method) = is_db_write(node, source) {
        writes.push((method, node));
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            find_db_writes_recursive(child, source, writes);
        }
    }
}

fn extract_assigned_var(node: Node, source: &str) -> String {
    match node.kind() {
        "variable_declaration" | "lexical_declaration" => {
            // const x = await prisma.user.findUnique(...)
            if let Some(declarator) = node.child(0) {
                if let Some(name_node) = declarator.child_by_field_name("name") {
                    return node_text(name_node, source).to_string();
                }
            }
        }
        "assignment_expression" => {
            // x = await prisma.user.findUnique(...)
            if let Some(left) = node.child_by_field_name("left") {
                return node_text(left, source).to_string();
            }
        }
        _ => {}
    }
    String::new()
}

fn extract_entity_from_read(source: &str, node: Node) -> Option<String> {
    let text = node_text(node, source);
    // Extract entity from patterns like "prisma.user.findUnique" or "tx.stockLevel.findFirst"
    let parts: Vec<&str> = text.split('.').collect();
    if parts.len() >= 2 {
        // The entity is typically the second-to-last part before the method
        // e.g., "prisma.user.findUnique" -> "user"
        // e.g., "tx.stockLevel.findFirst" -> "stockLevel"
        if parts.len() >= 3 {
            return Some(parts[parts.len() - 2].to_string());
        }
    }
    None
}

fn extract_entity_from_write(source: &str, node: Node) -> Option<String> {
    extract_entity_from_read(source, node)
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len - 3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_ts(source: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        parser.parse(source, None).unwrap()
    }

    #[test]
    fn test_detects_basic_toctou() {
        let source = r#"
async function createOrder(items: OrderItem[]) {
    for (const item of items) {
        const stock = await prisma.stockLevel.findFirst({
            where: { variantId: item.variantId, qtyOnHand: { gte: item.quantity } }
        });
        if (!stock) throw new Error('OUT_OF_STOCK');
    }
    return prisma.$transaction(async (tx) => {
        for (const item of items) {
            await tx.stockLevel.update({
                where: { variantId: item.variantId },
                data: { qtyOnHand: { decrement: item.quantity } }
            });
        }
    });
}
"#;
        let tree = parse_ts(source);
        let findings = CheckThenAct.scan(tree.root_node(), source, "test.ts");
        assert!(!findings.is_empty(), "should detect TOCTOU in basic pattern");
        assert_eq!(findings[0].pattern_id, "CHECK_THEN_ACT_TOCTOU");
        assert_eq!(findings[0].severity, "Critical");
    }

    #[test]
    fn test_no_false_positive_with_transaction() {
        let source = r#"
async function safe() {
    return prisma.$transaction(async (tx) => {
        const stock = await tx.stockLevel.findFirst({ where: { id } });
        if (!stock) throw new Error('NOT_FOUND');
        await tx.stockLevel.update({ where: { id }, data: { qty: 0 } });
    });
}
"#;
        let tree = parse_ts(source);
        let findings = CheckThenAct.scan(tree.root_node(), source, "test.ts");
        assert!(findings.is_empty(), "should not flag when inside $transaction");
    }

    #[test]
    fn test_no_false_positive_without_db_ops() {
        let source = r#"
function process(x: number) {
    if (x > 0) {
        return x * 2;
    }
    return 0;
}
"#;
        let tree = parse_ts(source);
        let findings = CheckThenAct.scan(tree.root_node(), source, "test.ts");
        assert!(findings.is_empty(), "should not flag non-database code");
    }

    #[test]
    fn test_detects_coupon_toctou() {
        let source = r#"
async function applyCoupon(code: string) {
    const coupon = await prisma.coupon.findUnique({ where: { code } });
    if (coupon.usageLimit && coupon.usedCount >= coupon.usageLimit) {
        throw new Error('COUPON_EXHAUSTED');
    }
    await prisma.coupon.update({
        where: { code },
        data: { usedCount: { increment: 1 } }
    });
}
"#;
        let tree = parse_ts(source);
        let findings = CheckThenAct.scan(tree.root_node(), source, "test.ts");
        assert!(!findings.is_empty(), "should detect coupon TOCTOU");
    }
}
