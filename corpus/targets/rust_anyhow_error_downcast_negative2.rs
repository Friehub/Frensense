// SAFE: Uses match on downcast to handle both the specific error and unexpected errors gracefully.

use anyhow::Error;

#[derive(Debug)]
struct MyError;

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "my error")
    }
}

impl std::error::Error for MyError {}

fn handle_error(err: Error) {
    match err.downcast::<MyError>() {
        Ok(specific) => println!("handled: {}", specific),
        Err(original) => println!("unknown error: {}", original),
    }
}
