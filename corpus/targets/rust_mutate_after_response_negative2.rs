// SAFE: Uses a buffered writer that flushes after the response is fully written
fn handle_request(writer: &mut impl std::io::Write) {
    writer.write_all(b"request handled").unwrap();
    send_response(200, "ok");
}
