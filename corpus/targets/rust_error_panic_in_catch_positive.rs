// [frensense]
// observation: `std::panic::catch_unwind` captures a panic but `resume_unwind` is not called on the error, allowing the thread to continue in an inconsistent or corrupted state.
// impact: After catching a panic, the thread may access corrupted data or skip critical cleanup, leading to undefined behavior or security vulnerabilities.
// improvement: Always call `resume_unwind` or properly handle the panic error, or avoid catching panics for non-recovery scenarios.

use std::panic;

fn process_request(input: &str) -> String {
    let result = panic::catch_unwind(|| {
        if input.is_empty() {
            panic!("empty input");
        }
        format!("processed: {}", input)
    });
    match result {
        Ok(val) => val,
        Err(_panic_err) => {
            "fallback response".into()
        }
    }
}

fn main() {
    let _ = process_request("hello");
    let _ = process_request("");
}
