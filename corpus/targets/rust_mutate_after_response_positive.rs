// Rule: RUST_MUTATE_AFTER_RESPONSE
fn handle_request() {
    send_response(200, "ok");
    write("request handled"); // Mutation after response
}
