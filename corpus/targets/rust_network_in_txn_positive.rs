// Rule: RUST_NETWORK_IN_TXN
fn process_order() {
    begin_transaction();
    let resp = fetch("https://example.com"); // Network call inside transaction
    commit();
}
