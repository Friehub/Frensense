// Rule: RUST_NETWORK_IN_TXN (negative — no rule expected)
fn process_order() {
    let resp = fetch("https://example.com"); // Network call outside transaction
    begin_transaction();
    update_db();
    commit();
}
