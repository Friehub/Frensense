use crate::Advisory;

pub fn find(snap: &crate::engine::project::FileSnapshot) -> Vec<Advisory> {
    let ext = snap.path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let chain =
        frensense_engine::cfg::def_use::build_def_use(snap.tree.root_node(), &snap.content, ext);

    let param_names = collect_param_names(snap);

    chain
        .definitions
        .iter()
        .enumerate()
        .filter_map(|(def_idx, def)| {
            if chain.uses_of_def(def_idx).is_empty()
                && !def.name.starts_with('_')
                && !param_names.contains(&def.name)
            {
                let line = crate::to_u32(snap.content[..def.start_byte].lines().count());
                // Find the full line to remove
                let line_start = snap.content[..def.start_byte]
                    .rfind('\n')
                    .map(|i| i + 1)
                    .unwrap_or(0);
                let line_end = snap.content[def.start_byte..]
                    .find('\n')
                    .map(|i| def.start_byte + i)
                    .unwrap_or(snap.content.len());
                let line_text = &snap.content[line_start..line_end];
                let needs_leading_newline =
                    line_start > 0 && snap.content.as_bytes().get(line_start - 1) == Some(&b'\n');
                let replacement = if needs_leading_newline { "" } else { "\n" };

                Some(
                    Advisory::bare(
                        "UNUSED_VARIABLE",
                        crate::Severity::Info,
                        snap.id,
                        &snap.path,
                        format!("Variable '{}' is defined but never used.", def.name),
                    )
                    .with_line(line)
                    .with_bytes(crate::to_u32(line_start), crate::to_u32(line_end))
                    .with_content(line_text)
                    .with_impact("Unused variables clutter code and may indicate incomplete logic.")
                    .with_improvement(
                        "Remove the variable or prefix with `_` if intentionally unused.",
                    )
                    .with_tags(["dead-code", "quality"])
                    .with_replacement(replacement),
                )
            } else {
                None
            }
        })
        .collect()
}

fn collect_param_names(
    snap: &crate::engine::project::FileSnapshot,
) -> std::collections::HashSet<String> {
    let mut params = std::collections::HashSet::new();
    let mut cursor = snap.tree.root_node().walk();
    'walk: loop {
        let node = cursor.node();
        let kind = node.kind();
        if kind == "formal_parameters" || kind == "parameters" || kind == "parameter_list" {
            let mut c2 = node.walk();
            for child in node.children(&mut c2) {
                let ck = child.kind();
                if ck == "identifier"
                    || ck == "parameter"
                    || ck == "required_parameter"
                    || ck == "optional_parameter"
                {
                    params.insert(snap.content[child.start_byte()..child.end_byte()].to_string());
                }
            }
        }
        if !cursor.goto_first_child() {
            loop {
                if cursor.goto_next_sibling() {
                    break;
                }
                if !cursor.goto_parent() {
                    break 'walk;
                }
            }
        }
    }
    params
}
