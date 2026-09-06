// SPDX-License-Identifier: MIT

use frensense::semantics::data_flow::normalization::{SemanticExtractor, SemanticOp};
use tree_sitter::Parser;

fn parse_ts(source: &str) -> tree_sitter::Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .unwrap();
    parser.parse(source, None).unwrap()
}

fn parse_rs(source: &str) -> tree_sitter::Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    parser.parse(source, None).unwrap()
}

#[test]
fn test_binding_equivalence() {
    let ts_source = "const x = 10;";
    let rs_source = "let x = 10;";

    let ts_tree = parse_ts(ts_source);
    let rs_tree = parse_rs(rs_source);

    let ts_node = ts_tree.root_node().child(0).unwrap();
    let rs_node = rs_tree.root_node().child(0).unwrap();

    let ts_ops = SemanticExtractor::extract(ts_node, ts_source, "ts");
    let rs_ops = SemanticExtractor::extract(rs_node, rs_source, "rs");

    assert_eq!(ts_ops.len(), 1);
    assert_eq!(rs_ops.len(), 1);

    match (&ts_ops[0], &rs_ops[0]) {
        (SemanticOp::Binding { name: ts_name, .. }, SemanticOp::Binding { name: rs_name, .. }) => {
            assert_eq!(&ts_name[..], "x");
            assert_eq!(&rs_name[..], "x");
        }
        _ => panic!("Expected Binding operations"),
    }
}

#[test]
fn test_assignment_equivalence() {
    let ts_source = "x = 20;";
    let rs_source = "x = 20;";

    let ts_tree = parse_ts(ts_source);
    let rs_tree = parse_rs(rs_source);

    let ts_node = ts_tree.root_node().child(0).unwrap();
    let rs_node = rs_tree.root_node().child(0).unwrap();

    let ts_ops = SemanticExtractor::extract(ts_node, ts_source, "ts");
    let rs_ops = SemanticExtractor::extract(rs_node, rs_source, "rs");

    assert_eq!(ts_ops.len(), 1);
    assert_eq!(rs_ops.len(), 1);

    match (&ts_ops[0], &rs_ops[0]) {
        (
            SemanticOp::Assignment {
                target: ts_target, ..
            },
            SemanticOp::Assignment {
                target: rs_target, ..
            },
        ) => {
            assert_eq!(&ts_target[..], "x");
            assert_eq!(&rs_target[..], "x");
        }
        _ => panic!("Expected Assignment operations"),
    }
}

#[test]
fn test_call_equivalence() {
    let ts_source = "foo(1, 2);";
    let rs_source = "foo(1, 2);";

    let ts_tree = parse_ts(ts_source);
    let rs_tree = parse_rs(rs_source);

    let ts_node = ts_tree.root_node().child(0).unwrap();
    let rs_node = rs_tree.root_node().child(0).unwrap();

    let ts_ops = SemanticExtractor::extract(ts_node, ts_source, "ts");
    let rs_ops = SemanticExtractor::extract(rs_node, rs_source, "rs");

    assert_eq!(ts_ops.len(), 1);
    assert_eq!(rs_ops.len(), 1);

    match (&ts_ops[0], &rs_ops[0]) {
        (
            SemanticOp::Call {
                function_name: ts_fn,
                args: ts_args,
                ..
            },
            SemanticOp::Call {
                function_name: rs_fn,
                args: rs_args,
                ..
            },
        ) => {
            assert_eq!(&ts_fn[..], "foo");
            assert_eq!(&rs_fn[..], "foo");
            assert_eq!(ts_args.len(), 2);
            assert_eq!(rs_args.len(), 2);
        }
        _ => panic!("Expected Call operations"),
    }
}
