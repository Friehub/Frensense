// Rule: RUST_MUTATE_AFTER_RESPONSE (negative — no rule expected)
fn handle_request() {
    write_log("processing"); // Mutation before response
    send_response(200, "ok");
}
