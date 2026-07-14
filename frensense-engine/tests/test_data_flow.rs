use std::fs;
use tree_sitter::Parser;

#[test]
fn test_extracted_flows() {
    let source =
        fs::read_to_string("../corpus/targets/ts_race_condition_read_check_write_positive.ts")
            .unwrap();
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .unwrap();
    let tree = parser.parse(&source, None).unwrap();

    let flows = frensense_engine::corpus::data_flow_extractor::extract_data_flows(
        tree.root_node(),
        &source,
    );
    println!("EXTRACTED_FLOWS: {:#?}", flows);
    assert!(!flows.is_empty());
}
