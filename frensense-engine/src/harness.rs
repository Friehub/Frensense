// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Shared analysis harness: the ONE implementation of "source text → lowered,
//! SSA'd, interprocedurally-linked program graph" used by the scanner, the
//! e2e drivers, and the bundler's fact extractor.
//!
//! The bundler MUST lower corpus pairs through this same harness (not its own
//! walker) so that facts are extracted from exactly the IR the engine scans.

use rustc_hash::FxHashMap;

use crate::ir::function::*;
use crate::ir::lowering::LoweringContext;
use crate::ir::ssa::SSABuilder;

/// Lower a single source file into a set of SSA'd `FunctionIR`s keyed by
/// function name.
///
/// Handles the module shapes that matter for app code:
/// * `function_declaration` / `method_definition` bodies,
/// * arrow/function-expression handlers registered via
///   `app.get|post|put|delete|use|all(path, handler)`, the dominant
///   Express-style shape,
/// * nested arrows inside those handlers.
///
/// Handler names are synthetic and file-position-stable
/// (`<path:handler@byte>`), so bundles derived from the same file hash to the
/// same keys.
pub fn lower_source(
    path: &str,
    source: &str,
    ext: &str,
) -> Result<FxHashMap<String, FunctionIR>, String> {
    let spec = frensense_lang::spec_for_ext(ext)
        .ok_or_else(|| format!("no language spec for extension '{ext}'"))?;

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&spec.tree_sitter_language())
        .map_err(|e| format!("parser init failed: {e}"))?;
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| format!("parse failed for {path}"))?;

    let mut irs: FxHashMap<String, FunctionIR> = FxHashMap::default();
    let mut stack = vec![tree.root_node()];
    while let Some(node) = stack.pop() {
        // Language-driven function detection: JS/TS `function_declaration` /
        // `method_definition`, Python `function_definition` (+ async),
        // Rust `function_item`, Go `function_declaration` / `method_declaration`.
        if spec.is_function_node(node.kind()) {
            let name = node
                .child_by_field_name("name")
                .map(|n| source[n.start_byte()..n.end_byte()].to_string())
                .unwrap_or_else(|| format!("<fn@{}>", node.start_byte()));
            let ir = lower_one(&name, node, source, spec);
            irs.entry(ir.name.clone()).or_insert(ir);
        }
        // Top-level arrow functions bound to consts/lets, the dominant
        // shape for utility libraries (`export const hash = (d) => ...`).
        if node.kind() == "lexical_declaration" || node.kind() == "variable_declaration" {
            let mut dc = node.walk();
            for decl in node.named_children(&mut dc) {
                if decl.kind() != "variable_declarator" {
                    continue;
                }
                if std::env::var("FRENSdbg_STATIC").is_ok() {
                    eprintln!(
                        "[static] decl kind={} value={:?}",
                        decl.kind(),
                        decl.child_by_field_name("value").map(|v| v.kind())
                    );
                }
                let Some(value) = decl.child_by_field_name("value") else {
                    continue;
                };
                let name = decl
                    .child_by_field_name("name")
                    .map(|n| source[n.start_byte()..n.end_byte()].to_string())
                    .unwrap_or_else(|| format!("<fn@{}>", decl.start_byte()));
                if value.kind() == "arrow_function" || value.kind() == "function_expression" {
                    let ir = lower_one(&name, value, source, spec);
                    irs.entry(ir.name.clone()).or_insert(ir);
                } else {
                    // Non-function static initializer: allowlist Sets, config
                    // objects, schema builders. Lowered as pseudo-IR so the
                    // checker layer can assert policy over them.
                    let ir = lower_static(&name, value, source, spec);
                    irs.entry(ir.name.clone()).or_insert(ir);
                }
            }
        }
        // Route-registration handlers: `app.get("/", (req, res) => ...)`.
        if node.kind() == "call_expression"
            && let (Some(callee), Some(args)) = (
                node.child_by_field_name("function"),
                node.child_by_field_name("arguments"),
            )
        {
            let callee_text = &source[callee.start_byte()..callee.end_byte()];
            if is_route_registration(callee_text) {
                let mut ac = args.walk();
                let arg_nodes: Vec<_> = args.named_children(&mut ac).collect();
                for a in &arg_nodes {
                    if a.kind() == "arrow_function" || a.kind() == "function_expression" {
                        let name = format!("<{}:handler@{}>", path, a.start_byte());
                        let ir = lower_one(&name, *a, source, spec);
                        irs.entry(ir.name.clone()).or_insert(ir);
                    }
                }
            }
        }
        let mut c = node.walk();
        let kids: Vec<_> = node.children(&mut c).collect();
        for k in kids.into_iter().rev() {
            stack.push(k);
        }
    }
    Ok(irs)
}

fn is_route_registration(callee_text: &str) -> bool {
    [".post", ".get", ".put", ".delete", ".use", ".all"]
        .iter()
        .any(|s| callee_text.ends_with(s))
}

fn lower_one(
    name: &str,
    fn_node: tree_sitter::Node,
    source: &str,
    spec: &'static dyn frensense_lang::spec::LanguageSpec,
) -> FunctionIR {
    let mut ctx = LoweringContext::new(spec, source, name.to_string());

    let params = fn_node.child_by_field_name("parameters");
    if let Some(params) = params {
        bind_params(&mut ctx, params, source);
    }
    if let Some(body) = fn_node.child_by_field_name("body") {
        ctx.visit_node(body);
    }
    SSABuilder::new(ctx.ir).build()
}

/// Lower a top-level non-function initializer (`const x = new Set([...])`,
/// `const cfg = { ... }`, `const tools = { ... tool({...}) ... }`) into a
/// pseudo-function IR so non-dataflow checkers can assert policy over
/// static configuration, allowlist definitions, schema builders, feature
/// flags, that no function body contains.
fn lower_static(
    name: &str,
    value_node: tree_sitter::Node,
    source: &str,
    spec: &'static dyn frensense_lang::spec::LanguageSpec,
) -> FunctionIR {
    let mut ctx = LoweringContext::new(spec, source, name.to_string());
    ctx.visit_node(value_node);
    SSABuilder::new(ctx.ir).build()
}

fn bind_params(ctx: &mut LoweringContext, params: tree_sitter::Node, source: &str) {
    let mut pc = params.walk();
    for p in params.children(&mut pc) {
        // Language-dependent parameter shapes:
        // - JS/TS: `required_parameter`/`optional_parameter` with `pattern` field
        // - Python: plain `identifier` children of `parameters`
        // - Rust: `parameter` node wrapping the name identifier as first child
        // - Go: `parameter_declaration` with the name as first child
        let id = match p.kind() {
            "required_parameter" | "optional_parameter" => {
                p.child_by_field_name("pattern").or_else(|| p.child(0))
            }
            "identifier" => Some(p),
            "parameter" | "parameter_declaration" | "variadic_parameter" => p.child(0),
            _ => None,
        };
        if let Some(id) = id {
            let name = source[id.start_byte()..id.end_byte()].to_string();
            let v = ctx.ir.new_var(VarMetadata {
                source_name: Some(name.clone()),
                type_name: None,
                byte_range: Some((p.start_byte(), p.end_byte())),
                is_memory_state: false,
                object_keys: Vec::new(),
            });
            ctx.ir.parameters.push(v);
            ctx.env.last_mut().unwrap().insert(name, v);
        }
    }
}
