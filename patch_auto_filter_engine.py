import re

content = open("frensense-engine/src/auto_filter.rs").read()

ast_extractor = """
pub fn extract_call_targets(source: &str) -> HashSet<String> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_typescript::language_typescript()).unwrap();
    let tree = parser.parse(source, None).unwrap();
    let node = tree.root_node();
    
    let mut targets = std::collections::HashSet::new();
    let mut visit_stack = vec![node];
    
    while let Some(n) = visit_stack.pop() {
        if n.kind() == "call_expression" {
            if let Some(callee) = n
                .child_by_field_name("function")
                .or_else(|| n.child_by_field_name("callee"))
            {
                if let Ok(text) =
                    std::str::from_utf8(&source.as_bytes()[callee.start_byte()..callee.end_byte()])
                {
                    targets.insert(text.to_string());
                }
            }
        }
        
        let mut child_cursor = n.walk();
        for child in n.children(&mut child_cursor) {
            visit_stack.push(child);
        }
    }
    targets
}
"""

content = re.sub(r'pub fn extract_call_targets.*?\}\n\}', ast_extractor.strip(), content, flags=re.DOTALL)
open("frensense-engine/src/auto_filter.rs", "w").write(content)

