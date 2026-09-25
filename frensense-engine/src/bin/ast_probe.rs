// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

// Probe: dump named node kinds of a TS file to see what the lowering misses.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let src = std::fs::read_to_string(&args[1]).unwrap();
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .unwrap();
    let tree = parser.parse(&src, None).unwrap();
    let mut stack = vec![(tree.root_node(), 0)];
    while let Some((n, d)) = stack.pop() {
        if n.is_named() {
            let txt = &src[n.start_byte()..n.end_byte().min(n.start_byte() + 60)];
            println!(
                "{}{} [{}] {:?}",
                "  ".repeat(d),
                n.kind(),
                n.child_by_field_name("name")
                    .map(|c| &src[c.start_byte()..c.end_byte()])
                    .unwrap_or(""),
                txt.replace('\n', " ")
            );
        }
        let mut c = n.walk();
        let kids: Vec<_> = n.children(&mut c).collect();
        for k in kids.into_iter().rev() {
            stack.push((k, d + 1));
        }
    }
}
