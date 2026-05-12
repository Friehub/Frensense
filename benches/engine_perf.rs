use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gensense::semantics::{Symbol, SymbolKind, SymbolRegistry};

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
        });
    }

    c.bench_function("find_at_100k", |b| {
        b.iter(|| {
            registry.find_at(
                black_box("sym_50000"),
                black_box("bench.rs"),
                black_box(50001),
            )
        })
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
        })
        .collect();

    c.bench_function("assembly_50k_symbols", |b| {
        b.iter(|| {
            let mut registry = SymbolRegistry::new();
            for sym in &symbols {
                registry.insert(sym.clone());
            }
            black_box(registry)
        })
    });
}

criterion_group!(benches, bench_symbol_lookup, bench_assembly_phase);
criterion_main!(benches);
