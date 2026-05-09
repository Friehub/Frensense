use gensense::{Advisory, GenSenseAuditor, GenSenseContext, GenSenseRule, Severity};
use std::path::Path;
use tree_sitter::Parser;

struct TaintRule;
impl GenSenseRule for TaintRule {
    fn id(&self) -> &str {
        "TAINT_TEST"
    }
    fn description(&self) -> &str {
        "Detects tainted data leaks"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn check(&self, _node: tree_sitter::Node, _context: &GenSenseContext) -> Vec<Advisory> {
        vec![]
    }
}

#[test]
fn test_interprocedural_taint_leak() {
    let content = r#"
        fn get_user_secret() -> String { "secret".to_string() }

        fn main() {
            let secret = get_user_secret();
            process_data(secret);
        }

        fn process_data(data: String) {
            leak_to_log(data);
        }

        fn leak_to_log(val: String) {
            println!("{}", val);
        }
    "#;

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(content, None).unwrap();

    let mut registry = gensense::semantics::SymbolRegistry::new();
    let auditor = GenSenseAuditor::new(vec![]);

    let path = Path::new("test.rs");

    // 1. Discover symbols
    let syms = auditor.discover_symbols(path, content).unwrap();
    for sym in syms {
        registry.insert(sym);
    }

    // 2. Discover events and link data flow
    auditor
        .discover_events(path, content, &mut registry)
        .unwrap();

    let context = GenSenseContext {
        file_path: path,
        source_code: content,
        symbols: &registry,
    };

    let analyzer = gensense::semantics::DataFlowAnalyzer::new(&context, tree.root_node());

    let source_re = regex::Regex::new("get_user_secret").unwrap();
    let sink_re = regex::Regex::new("println").unwrap();

    println!("Graph Nodes:");
    for idx in registry.graph.all_nodes() {
        if let Some(node) = registry.graph.get_node(idx) {
            match node {
                gensense::semantics::graph::SemanticNode::Declaration(s) => {
                    println!("  Symbol: {} ({:?})", s.name, s.kind)
                }
                gensense::semantics::graph::SemanticNode::Event(e) => {
                    println!("  Event: {} ({:?})", e.label, e.event_type)
                }
            }
        }
    }

    let advisories = analyzer.check_taint_graph(&source_re, &sink_re, &TaintRule);

    assert!(
        !advisories.is_empty(),
        "Should have detected an inter-procedural taint leak"
    );
    assert!(
        advisories[0].observation.contains("println"),
        "Observation should mention the sink"
    );
}
