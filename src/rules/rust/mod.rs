// SPDX-License-Identifier: MIT

#[allow(clippy::module_inception)]
pub mod async_safety;
pub mod blocking_io;
pub mod deadlock_guard;
pub mod fake_async;
pub mod timeout_guard;
pub mod tracing_guard;

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
