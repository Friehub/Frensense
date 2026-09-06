import re
import sys

# 1. cross_file.rs
with open('src/semantics/data_flow/cross_file.rs', 'r') as f:
    c = f.read()
c = c.replace('let mut fn_name_field = fn_name_full;', 'let mut _fn_name_field = fn_name_full;')
c = c.replace('fn_name_field = &self.source[field.start_byte()..field.end_byte()];', '_fn_name_field = &self.source[field.start_byte()..field.end_byte()];')
c = c.replace('"conditional_expression" | "ternary_expression" => {', '/* unreachable */ _ => {')
with open('src/semantics/data_flow/cross_file.rs', 'w') as f:
    f.write(c)

# 2. middleware_audit.rs
with open('src/engine/findings/middleware_audit.rs', 'r') as f:
    c = f.read()
c = re.sub(r'cwe: &\x27static str,', '', c)
c = re.sub(r'cwe: ".*?",', '', c)
with open('src/engine/findings/middleware_audit.rs', 'w') as f:
    f.write(c)

# 3. runner.rs
with open('src/engine/project/runner.rs', 'r') as f:
    c = f.read()
c = c.replace('fn find_function_node<\'a>(', '#[allow(dead_code)]\nfn find_function_node<\'a>(')
c = c.replace('fn has_function_child(node: tree_sitter::Node<\'_>) -> bool {', '#[allow(dead_code)]\nfn has_function_child(node: tree_sitter::Node<\'_>) -> bool {')
with open('src/engine/project/runner.rs', 'w') as f:
    f.write(c)

# 4. reporting.rs
with open('src/cli/reporting.rs', 'r') as f:
    c = f.read()
c = c.replace('advisories.swap_remove(i);', 'let _ = advisories.swap_remove(i);')
with open('src/cli/reporting.rs', 'w') as f:
    f.write(c)

