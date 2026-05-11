// SPDX-License-Identifier: MIT

use super::normalization::{SemanticExtractor, SemanticOp};
use super::{DataFlowAnalyzer, TaintRegistry};
use crate::{Advisory, GenSenseRule};
use regex::Regex;
use tree_sitter::Node;

impl<'a, 'ctx> DataFlowAnalyzer<'a, 'ctx> {
    fn with_ops<F, R>(&self, node: Node<'a>, f: F) -> R
    where
        F: FnOnce(&[SemanticOp<'a>]) -> R,
    {
        let ext = self
            .context
            .file_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        // Fix: Avoid holding the borrow while calling the closure f
        let ops = {
            let mut cache = self.context.semantic_cache.borrow_mut();
            cache
                .entry(node.id())
                .or_insert_with(|| SemanticExtractor::extract(node, self.context.source_code, ext))
                .clone() // Clone for safety during recursion
        };

        f(&ops)
    }

    pub fn discover_symbols(&self, registry: &mut TaintRegistry<'a>) {
        self.with_ops(self.root, |ops| {
            for op in ops {
                if let SemanticOp::Binding { name, value_node } = op {
                    registry.register_symbol(name, *value_node);
                }
            }
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub fn analyze_block(
        &self,
        node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        mut registry: TaintRegistry<'a>,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        self.with_ops(node, |ops| {
            for op in ops {
                match op {
                    SemanticOp::Binding { name, value_node } => {
                        registry.register_symbol(name, *value_node);
                        let v_node = *value_node;
                        let v_code =
                            &self.context.source_code[v_node.start_byte()..v_node.end_byte()];
                        if source_re.is_match(v_code) {
                            registry.taint(name, "source");
                        } else if let Some(origin) = self.resolve_taint(v_node, &registry) {
                            registry.taint(name, origin);
                        }
                    }
                    SemanticOp::Assignment { target, value_node } => {
                        let v_node = *value_node;
                        let v_code =
                            &self.context.source_code[v_node.start_byte()..v_node.end_byte()];
                        if source_re.is_match(v_code) {
                            registry.taint(target, "source");
                        } else if let Some(origin) = self.resolve_taint(v_node, &registry) {
                            registry.taint(target, origin);
                        }
                    }
                    SemanticOp::Call {
                        function_name,
                        args,
                        node: call_node,
                    } => {
                        advisories.extend(self.analyze_call(
                            function_name,
                            args,
                            *call_node,
                            source_re,
                            sink_re,
                            rule,
                            &mut registry,
                        ));
                    }
                    SemanticOp::EnterBlock(body_node) => {
                        if self.depth < self.max_depth {
                            registry.push_scope();
                            let sub_analyzer = DataFlowAnalyzer::with_depth(
                                self.context,
                                *body_node,
                                self.depth + 1,
                                self.max_depth,
                            );
                            advisories.extend(sub_analyzer.analyze_block(
                                *body_node,
                                source_re,
                                sink_re,
                                rule,
                                registry.clone(),
                            ));
                            registry.pop_scope();
                        }
                    }
                }
            }
        });

        advisories
    }

    #[allow(clippy::too_many_arguments)]
    fn analyze_call(
        &self,
        fn_name: &'a str,
        args: &[Node<'a>],
        _call_node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry<'a>,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut tainted_args = Vec::new();

        for (idx, arg) in args.iter().enumerate() {
            if let Some(origin) = self.resolve_taint(*arg, registry) {
                tainted_args.push(idx);
                if sink_re.is_match(fn_name) {
                    let arg_code = &self.context.source_code[arg.start_byte()..arg.end_byte()];
                    advisories.push(rule.new_advisory(
                        arg,
                        self.context,
                        format!("Inter-procedural Leak: Tainted data from '{origin}' reached sink '{fn_name}' via variable '{arg_code}'."),
                    ));
                }
            }
        }

        if !tainted_args.is_empty() && self.depth < self.max_depth {
            if let Some(def_node) = self.find_definition(fn_name, registry) {
                if let Some(next_registry) = self.map_params(def_node, &tainted_args) {
                    if let Some(body) = def_node.child_by_field_name("body") {
                        let sub_analyzer = DataFlowAnalyzer::with_depth(
                            self.context,
                            body,
                            self.depth + 1,
                            self.max_depth,
                        );
                        advisories.extend(sub_analyzer.analyze_block(
                            body,
                            source_re,
                            sink_re,
                            rule,
                            next_registry,
                        ));
                    }
                }
            }
        }

        advisories
    }

    fn resolve_taint(&self, node: Node<'a>, registry: &TaintRegistry<'a>) -> Option<&'a str> {
        if node.kind() == "identifier" {
            let name = &self.context.source_code[node.start_byte()..node.end_byte()];
            return registry.get_origin(name);
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(origin) = self.resolve_taint(child, registry) {
                return Some(origin);
            }
        }
        None
    }
}
