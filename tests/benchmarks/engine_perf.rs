use criterion::{Criterion, black_box, criterion_group, criterion_main};
use gensense::Engine;
use gensense::engine::auditor::GenSenseAuditor;
use gensense::semantics::{Symbol, SymbolKind, SymbolRegistry};
use std::path::Path;

// ─── Existing: SRI Symbol Registry ──────────────────────────────────────────

fn bench_symbol_lookup(c: &mut Criterion) {
    let mut registry = SymbolRegistry::new();
    let num_symbols = 100_000;

    for i in 0..num_symbols {
        registry.insert(Symbol {
            name: format!("sym_{i}"),
            kind: SymbolKind::Function,
            start_byte: 0,
            end_byte: 10,
            line: i + 1,
            end_line: i + 1,
            column: 1,
            file_path: "bench.rs".to_string(),
            file_id: gensense::FileId(0),
        });
    }

    c.bench_function("find_at_100k", |b| {
        b.iter(|| {
            registry.find_at(
                black_box("sym_50000"),
                black_box("bench.rs"),
                black_box(50001),
            )
        });
    });
}

fn bench_assembly_phase(c: &mut Criterion) {
    let num_symbols = 50_000;
    let symbols: Vec<_> = (0..num_symbols)
        .map(|i| Symbol {
            name: format!("sym_{i}"),
            kind: SymbolKind::Function,
            start_byte: 0,
            end_byte: 10,
            line: i + 1,
            end_line: i + 1,
            column: 1,
            file_path: format!("file_{}.rs", i / 10),
            file_id: gensense::FileId(0),
        })
        .collect();

    c.bench_function("assembly_50k_symbols", |b| {
        b.iter(|| {
            let mut registry = SymbolRegistry::new();
            for sym in &symbols {
                registry.insert(sym.clone());
            }
            black_box(registry)
        });
    });
}

fn bench_sri_lookup(c: &mut Criterion) {
    let mut registry = SymbolRegistry::new();
    let num_symbols = 50_000;
    let file_path = "large_file.ts";

    for i in 0..num_symbols {
        registry.insert(Symbol {
            name: format!("func_{i}"),
            kind: SymbolKind::Function,
            start_byte: i * 100,
            end_byte: (i * 100) + 50,
            line: i * 10 + 1,
            end_line: i * 10 + 5,
            column: 1,
            file_path: file_path.to_string(),
            file_id: gensense::FileId(0),
        });
    }

    c.bench_function("sri_find_function_at_50k", |b| {
        b.iter(|| {
            registry.find_function_at(
                black_box(file_path),
                black_box(250_000), // Middle of the file
            )
        });
    });
}

// ─── New: Rule Compilation Throughput ────────────────────────────────────────

fn bench_rule_compilation(c: &mut Criterion) {
    c.bench_function("compile_builtin_rules", |b| {
        b.iter(|| {
            let (rules, _) = GenSenseAuditor::default_rules();
            black_box(rules)
        });
    });
}

// ─── New: Full Scan Throughput ────────────────────────────────────────────────

fn bench_full_scan_throughput(c: &mut Criterion) {
    let source = include_str!("../../src/lib.rs");
    c.bench_function("full_scan_lib_rs", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            let advisories =
                engine.run_content(black_box(Path::new("src/lib.rs")), black_box(source));
            black_box(advisories)
        });
    });
}

// ─── New: Patcher Throughput ─────────────────────────────────────────────────

fn bench_patcher_throughput(c: &mut Criterion) {
    use gensense::patcher::PatchManager;
    use gensense::{Advisory, FileId};
    use std::io::Write;
    use tempfile::NamedTempFile;

    let mut content = String::with_capacity(15_000);
    for i in 0..499 {
        content.push_str(&format!("const x{i} = {i}; // padding line\n"));
    }
    content.push_str("const users = await prisma.user.findMany();\n");

    let mut tmp = NamedTempFile::new().expect("tempfile");
    tmp.write_all(content.as_bytes()).expect("write");
    let root = tmp.path().parent().unwrap();

    let original = "prisma.user";
    let start_byte = content.find(original).unwrap() as u32;
    let end_byte = start_byte + original.len() as u32;

    let advisory = Advisory {
        rule_id: "BENCH_FIX".into(),
        file_id: FileId(0),
        file_path: tmp.path().file_name().unwrap().to_string_lossy().into(),
        line: 500,
        column: 1,
        severity: gensense::Severity::Warning,
        observation: "bench".into(),
        impact: "bench".into(),
        improvement: "bench".into(),
        original_content: original.into(),
        proposed_replacement: Some("userService".into()),
        proposed_import: None,
        start_byte,
        end_byte,
        enclosing_symbol: None,
        confidence: 1.0,
        fingerprint: "bench".into(),
        auto_fixable: true,
        requires_human: false,
        tags: vec![],
    };

    let manager = PatchManager::new(root);

    c.bench_function("patcher_apply_fix_500line_file", |b| {
        b.iter(|| {
            std::fs::write(tmp.path(), &content).ok();
            let result = manager.apply_fix(
                black_box(&advisory),
                black_box(tmp.path().file_name().unwrap().as_ref()),
            );
            black_box(result)
        });
    });
}

criterion_group!(
    benches,
    bench_symbol_lookup,
    bench_assembly_phase,
    bench_sri_lookup,
    bench_rule_compilation,
    bench_full_scan_throughput,
    bench_patcher_throughput,
);
criterion_main!(benches);
