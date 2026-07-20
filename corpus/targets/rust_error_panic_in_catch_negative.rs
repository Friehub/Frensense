// SAFE: Resumes the unwind after logging, ensuring the panic propagates properly
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
        Err(panic_err) => {
            eprintln!("caught panic, propagating");
            panic::resume_unwind(panic_err);
        }
    }
}

fn main() {
    let _ = process_request("hello");
    let _ = process_request("");
}
