// SAFE: Pre-fetches network data before beginning the transaction
fn process_order() {
    let resp = fetch("https://example.com");
    begin_transaction();
    update_db();
    commit();
}
