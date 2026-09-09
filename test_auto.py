import re
content = open("frensense-engine/src/auto_filter.rs").read()
# Replace the while loop with a bounded while loop
content = re.sub(
    r'let mut visit_stack = vec!\[node\];\n\s*while let Some\(n\) = visit_stack.pop\(\) \{',
    'let mut visit_stack = vec![node];\n    let mut iters = 0;\n    while let Some(n) = visit_stack.pop() {\n        iters += 1;\n        if iters > 1000000 { panic!("Infinite loop in extract_call_targets"); }\n',
    content
)
open("frensense-engine/src/auto_filter.rs", "w").write(content)
