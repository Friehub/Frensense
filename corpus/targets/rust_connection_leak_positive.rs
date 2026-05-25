// Rule: RUST_CONNECTION_LEAK
fn fetch_data() {
    let conn = get_connection();
    // conn is never closed or released — leak
    conn.query("SELECT 1");
}
