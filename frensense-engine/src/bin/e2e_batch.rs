// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

// End-to-end batch runner: full pipeline over the corpus targets.
// Each target file is an independent program. Classification comes from the
// filename convention: *_positive.ts → expect ≥1 alert; *_negative.ts →
// expect 0 alerts.

use frensense_engine::analysis::forward::ProgramSvfg;
use frensense_engine::analysis::taint::config::TaintConfig;
use frensense_engine::analysis::taint::engine::BackwardTaintEngine;
use frensense_engine::analysis::taint::facts::{FactTable, SinkSignature};
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
            "find",
            "findOne",
            "findAndModify",
            "updateOne",
            "updateMany",
            "deleteOne",
            "deleteMany",
            "where",
            "sendFile",
            "send",
            "redirect",
            "render",
            "runInContext",
            "openExternal",
            "writeFile",
            "readFile",
            "request",
            "get",
            "post",
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

fn arrow_name(path: &str, node: tree_sitter::Node) -> String {
    format!("<{}:handler@{}>", path, node.start_byte())
}

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
}

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

fn lower_file(
    path: &str,
    source: &str,
    spec: &'static dyn frensense_lang::spec::LanguageSpec,
) -> Vec<FunctionIR> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .expect("lang");
    let tree = match parser.parse(source, None) {
        Some(t) => t,
        None => return Vec::new(),
    };

    let mut irs = Vec::new();
    let mut stack = vec![tree.root_node()];
    while let Some(node) = stack.pop() {
        match node.kind() {
            "function_declaration" | "method_definition" => {
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
                if let (Some(callee), Some(args)) = (
                    node.child_by_field_name("function"),
                    node.child_by_field_name("arguments"),
                ) {
                    let callee_text = &source[callee.start_byte()..callee.end_byte()];
                    let is_reg = [".post", ".get", ".use", ".put", ".delete", ".all"]
                        .iter()
                        .any(|s| callee_text.ends_with(s));
                    if is_reg {
                        let mut ac = args.walk();
                        let arg_nodes: Vec<_> = args.named_children(&mut ac).collect();
                        for a in &arg_nodes {
                            if a.kind() == "arrow_function" || a.kind() == "function_expression" {
                                irs.push(lower_fn(
                                    &arrow_name(path, *a),
                                    a.child_by_field_name("parameters"),
                                    a.child_by_field_name("body"),
                                    source,
                                    spec,
                                ));
                            }
                        }
                    }
                }
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

/// Built-in fact table: sink signatures encoding per-argument danger.
/// This is what frensense-lang tables will supply (JS provider), plus what a
/// .frc bundle can merge over it.
fn ts_fact_table() -> FactTable {
    let mut t = FactTable::default();
    // Parameterized queries: slot 0 is the SQL template (dangerous when
    // tainted by interpolation), slot 1 is the binding array (safe channel).
    for call in ["query", "execute"] {
        let mut sig = SinkSignature::with_args(call, &[0]);
        sig.binding_args_safe = true;
        t.sink_signatures.insert(call.into(), sig);
    }
    // Guard-style / transform sanitizers (what the JS provider's table will
    // carry; here inlined for the e2e measurement).
    for (call, kind, guard) in [
        ("test", "allowlist", true),    // SAFE_RE.test(x)
        ("includes", "validate", true), // !x.includes('..') guard
        ("startsWith", "validate", true),
        ("replace", "encode", false), // x.replace(/re/g, '')
        ("replaceAll", "encode", false),
        ("parseInt", "coerce", false),
        ("toString", "coerce", false),
        ("trim", "encode", false),
    ] {
        t.sanitizer_facts.insert(
            call.into(),
            frensense_engine::analysis::taint::facts::SanitizerFact {
                call: call.into(),
                kind: kind.into(),
                sanitizes_args: Default::default(),
                guard_style: guard,
            },
        );
    }
    t
}

/// Analyze one file; returns (alerts, parse_ok, fn_count).
fn analyze(path: &str, src: &str) -> (usize, bool, usize) {
    let spec = match frensense_lang::registry::spec_for_ext("ts") {
        Some(s) => s,
        None => return (0, false, 0),
    };
    let irs = lower_file(path, src, spec);
    if irs.is_empty() {
        return (0, true, 0);
    }
    let mut all: FxHashMap<String, FunctionIR> = FxHashMap::default();
    for ir in irs {
        all.entry(ir.name.clone()).or_insert(ir);
    }
    let names: Vec<String> = all.keys().cloned().collect();
    let mut statics: FxHashMap<String, &'static FunctionIR> = FxHashMap::default();
    for n in &names {
        let ir = all.remove(n).unwrap();
        statics.insert(n.clone(), Box::leak(Box::new(ir)));
    }
    let mut config = ts_config();
    // Spec-driven built-ins: merge the language spec's knowledge (sources,
    // sinks, sanitizers) over the measurement baseline.
    if std::env::var("FRENS_SPEC_CONFIG").is_ok() {
        let sc = frensense_engine::analysis::taint::facts::config_from_spec(spec);
        config.sources.extend(sc.sources);
        config.sinks.extend(sc.sinks);
        config.sanitizers.extend(sc.sanitizers);
    }
    let mut facts = ts_fact_table();
    if std::env::var("FRENS_SPEC_FACTS").is_ok() {
        facts.merge(&frensense_engine::analysis::taint::facts::fact_table_from_spec(spec));
    }
    let prog = ProgramSvfg::new(&statics, &config);
    let mut engine = BackwardTaintEngine::new(&prog, &config).with_fact_table(&facts);
    engine.run();
    (
        engine.findings.iter().filter(|f| f.alert.is_some()).count(),
        true,
        statics.len(),
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("../corpus/targets");
    let mut files: Vec<String> = Vec::new();
    for e in fs::read_dir(dir).expect("dir") {
        let e = e.unwrap();
        let p = e.path();
        if p.extension()
            .map(|x| x == "ts" || x == "tsx")
            .unwrap_or(false)
        {
            files.push(p.to_string_lossy().into_owned());
        }
    }
    files.sort();

    let (mut tp, mut fp, mut fn_, mut tn, mut errors) = (0u32, 0u32, 0u32, 0u32, 0u32);
    let mut mismatches: Vec<(String, &'static str, usize)> = Vec::new();

    let t0 = std::time::Instant::now();
    for path in &files {
        let name = path.rsplit('/').next().unwrap_or(path);
        let expect_positive = name.contains("_positive");
        let expect_negative = name.contains("_negative");
        if !expect_positive && !expect_negative {
            continue;
        }
        let src = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => {
                errors += 1;
                continue;
            }
        };
        let (alerts, ok, nfns) = analyze(path, &src);
        if !ok {
            errors += 1;
            continue;
        }
        if expect_positive {
            if alerts > 0 {
                tp += 1;
            } else {
                fn_ += 1;
                mismatches.push((name.into(), "MISSED", nfns));
            }
        } else {
            if alerts > 0 {
                fp += 1;
                mismatches.push((name.into(), "FALSE-POS", alerts));
            } else {
                tn += 1;
            }
        }
    }
    let dt = t0.elapsed();

    let total = tp + fp + fn_ + tn;
    println!(
        "=== corpus e2e: {} files analyzed ({}) ===",
        total,
        dt.as_secs_f32()
    );
    println!("  TP (vuln found):        {}", tp);
    println!("  FN (vuln missed):       {}", fn_);
    println!("  FP (clean flagged):     {}", fp);
    println!("  TN (clean passed):      {}", tn);
    if tp + fn_ > 0 {
        println!("  recall:    {:.1}%", 100.0 * tp as f32 / (tp + fn_) as f32);
    }
    if tp + fp > 0 {
        println!("  precision: {:.1}%", 100.0 * tp as f32 / (tp + fp) as f32);
    }
    if total > 0 {
        println!(
            "  avg time:  {:.2} ms/file",
            dt.as_secs_f32() * 1000.0 / total as f32
        );
    }
    if errors > 0 {
        println!("  (skipped/errors: {})", errors);
    }

    println!("\n--- mismatches (all) ---");
    for (n, kind, v) in mismatches.iter().take(1000) {
        println!("  {:9} {} ({})", kind, n, v);
    }
    if mismatches.len() > 40 {
        println!("  ... and {} more", mismatches.len() - 40);
    }
}
