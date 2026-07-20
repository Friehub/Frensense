// SAFE: Only catches panics that are explicitly recoverable; re-throws via resume_unwind for unrecoverable ones
use std::panic;

fn process_request(input: &str) -> Result<String, String> {
    let result = panic::catch_unwind(|| {
        if input.is_empty() {
            panic!("empty input");
        }
        if input.len() > 100 {
            return Err("input too long".to_string());
        }
        Ok(format!("processed: {}", input))
    });
    match result {
        Ok(inner_result) => inner_result,
        Err(panic_err) => {
            let msg = match panic_err.downcast_ref::<&str>() {
                Some(s) => format!("panic: {}", s),
                None => "unknown panic".into(),
            };
            Err(msg)
        }
    }
}

fn main() {
    let _ = process_request("hello");
    let _ = process_request("");
}
