// Rule: RUST_CONNECTION_LEAK (negative — no rule expected)
fn fetch_data() {
    let conn = get_connection();
    conn.query("SELECT 1");
    close(conn); // Released properly
}
