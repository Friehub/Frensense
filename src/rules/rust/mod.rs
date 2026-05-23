// SPDX-License-Identifier: MIT

#[allow(clippy::module_inception)]
pub mod async_safety;
pub mod blocking_io;
pub mod deadlock_guard;
pub mod fake_async;
pub mod timeout_guard;
pub mod tracing_guard;

pub(crate) fn is_excluded_test_scope(node: tree_sitter::Node, context: &crate::GenSenseContext) -> bool {
    let file_path = context.file_path.to_string_lossy();
    if file_path.contains("tests/") || file_path.contains("tests-build/") {
        return true;
    }
    let mut current = node.parent();
    while let Some(ancestor) = current {
        let text = &context.source_code[ancestor.start_byte()..ancestor.end_byte()];
        if text.contains("#[cfg(test)]") || text.contains("#[test]") {
            return true;
        }
        current = ancestor.parent();
    }
    false
}

pub(crate) fn is_in_async_scope(node: tree_sitter::Node, source: &str) -> bool {
    let mut current = node;
    while let Some(parent) = current.parent() {
        let kind = parent.kind();
        if kind == "async_block" {
            return true;
        }
        if kind == "function_item" {
            let header = &source[parent.start_byte()
                ..parent
                    .child_by_field_name("body")
                    .map_or(parent.end_byte(), |b| b.start_byte())];
            return header.contains("async");
        }
        current = parent;
    }
    false
}
