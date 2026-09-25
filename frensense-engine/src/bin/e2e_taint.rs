// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

// End-to-end probe (v2): full pipeline over real corpus files, including
// arrow-function handlers registered via `app.post(path, handler)`.
// This version wires: arrow functions + params, member-call lowering,
// and the module-level handler registration shape.

use frensense_engine::analysis::forward::ProgramSvfg;
use frensense_engine::analysis::taint::config::TaintConfig;
use frensense_engine::ir::function::*;
use frensense_engine::ir::lowering::LoweringContext;
use frensense_engine::ir::ssa::SSABuilder;
use rustc_hash::FxHashMap;
use std::fs;

fn ts_config() -> TaintConfig {
    TaintConfig {
        sources: [
            "req.body",
            "req.query",
            "req.params",
            "req.headers",
            "request.body",
            "request.query",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
        sinks: [
            "exec",
            "execSync",
            "execAsync",
            "spawn",
            "query",
            "execute",
            "eval",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
        sanitizers: ["escapeHtml", "sanitize", "validate"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    }
}

/// Build a unique fn name for arrow handlers registered on a router/app.
fn arrow_name(path: &str, node: tree_sitter::Node) -> String {
    format!("<{}:handler@{}>", path, node.start_byte())
}

/// Register parameters from a `formal_parameters` node into the context.
fn bind_params(ctx: &mut LoweringContext, params: tree_sitter::Node, source: &str) {
    let mut pc = params.walk();
    for p in params.children(&mut pc) {
        if (p.kind() == "required_parameter" || p.kind() == "optional_parameter")
            && let Some(id) = p.child_by_field_name("pattern").or_else(|| p.child(0))
        {
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
    let _ = source;
}

/// Lower a function-like node (function_declaration or arrow_function).
fn lower_fn(
    name: &str,
    params: Option<tree_sitter::Node>,
    body: Option<tree_sitter::Node>,
    source: &str,
    spec: &'static dyn frensense_lang::spec::LanguageSpec,
) -> FunctionIR {
    let mut ctx = LoweringContext::new(spec, source, name.to_string());
    if let Some(params) = params {
        bind_params(&mut ctx, params, source);
    }
    if let Some(body) = body {
        ctx.visit_node(body);
    }
    SSABuilder::new(ctx.ir).build()
}

/// Walk a file. Lowers function declarations; for `app.post(path, arrow)`
/// registrations, lowers the arrow as its own function.
fn lower_file(
    path: &str,
    source: &str,
    spec: &'static dyn frensense_lang::spec::LanguageSpec,
) -> Vec<FunctionIR> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .expect("lang");
    let tree = parser.parse(source, None).expect("parse");

    let mut irs = Vec::new();
    let mut stack = vec![tree.root_node()];
    while let Some(node) = stack.pop() {
        match node.kind() {
            "function_declaration" => {
                let name = node
                    .child_by_field_name("name")
                    .map(|n| source[n.start_byte()..n.end_byte()].to_string())
                    .unwrap_or_else(|| format!("<fn@{}>", node.start_byte()));
                irs.push(lower_fn(
                    &name,
                    node.child_by_field_name("parameters"),
                    node.child_by_field_name("body"),
                    source,
                    spec,
                ));
            }
            "call_expression" => {
                // app.post("/path", (req, res) => { ... }), handler is arg 1.
                let callee = node.child_by_field_name("function");
                let args = node.child_by_field_name("arguments");
                if let (Some(callee), Some(args)) = (callee, args) {
                    let callee_text = source[callee.start_byte()..callee.end_byte()].to_string();
                    let is_reg = callee_text.ends_with(".post")
                        || callee_text.ends_with(".get")
                        || callee_text.ends_with(".use")
                        || callee_text.ends_with(".put")
                        || callee_text.ends_with(".delete")
                        || callee_text.ends_with(".all");
                    if is_reg {
                        let mut ac = args.walk();
                        let arg_nodes: Vec<_> = args.named_children(&mut ac).collect();
                        for a in &arg_nodes {
                            if a.kind() == "arrow_function" || a.kind() == "function_expression" {
                                let name = arrow_name(path, *a);
                                irs.push(lower_fn(
                                    &name,
                                    a.child_by_field_name("parameters"),
                                    a.child_by_field_name("body"),
                                    source,
                                    spec,
                                ));
                            }
                        }
                    }
                }
                // continue walking into args (nested arrows)
            }
            _ => {}
        }
        let mut c = node.walk();
        let kids: Vec<_> = node.children(&mut c).collect();
        for k in kids.into_iter().rev() {
            stack.push(k);
        }
    }
    irs
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: e2e_taint <file.ts> [...]");
        std::process::exit(1);
    }
    let config = ts_config();

    let mut all_irs: FxHashMap<String, &'static FunctionIR> = FxHashMap::default();
    let mut file_count = 0usize;

    for path in &args[1..] {
        let src = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("skip {path}: {e}");
                continue;
            }
        };
        file_count += 1;
        for ir in lower_file(
            path,
            &src,
            frensense_lang::registry::spec_for_ext("ts").unwrap(),
        ) {
            all_irs
                .entry(ir.name.clone())
                .or_insert(Box::leak(Box::new(ir)));
        }
    }

    println!(
        "lowered {} functions from {} files",
        all_irs.len(),
        file_count
    );
    let mut names: Vec<&String> = all_irs.keys().collect();
    names.sort();
    for name in &names {
        let ir = all_irs[name.as_str()];
        println!(
            "  fn {name}: {} instrs",
            ir.blocks
                .values()
                .map(|b| b.instructions.len())
                .sum::<usize>()
        );
    }

    let prog = ProgramSvfg::new(&all_irs, &config);
    for (i, f) in prog.functions.iter().enumerate() {
        for b in &f.bindings {
            println!(
                "  binding [{}] {} → {} (callees {:?})",
                i, f.name, b.callee_name, b.callees
            );
        }
    }

    // Dump cross edges.
    for ((fi, k), edges) in &prog.cross_edges {
        for (gf, gk) in edges {
            println!("  cross: fn{} {:?} -> fn{} {:?}", fi, k, gf, gk);
        }
    }

    // Trace the backward exploration for the FP case (spawn arg1).
    // Report which nodes saw a source.
    // Dump SVFG node kinds for the handler to debug traversal.
    for (i, f) in prog.functions.iter().enumerate() {
        let mut keys: Vec<_> = f.svfg.nodes.keys().collect();
        keys.sort_by_key(|k| (k.block.0, k.instr_idx, k.var.0));
        for k in keys {
            let n = f.svfg.node(k).unwrap();
            println!("  node [{}] {:?} kind={:?}", i, k, n.kind);
        }
    }

    use frensense_engine::analysis::taint::engine::BackwardTaintEngine;
    let mut engine = BackwardTaintEngine::new(&prog, &config);
    engine.run();
    println!("=== {} findings:", engine.findings.len());
    for f in &engine.findings {
        println!(
            "  [{}] {} arg#{} → {:?}",
            f.function, f.sink, f.arg_slot, f.verdict
        );
        if let Some(a) = &f.alert {
            println!("      {a}");
        }
    }
}
