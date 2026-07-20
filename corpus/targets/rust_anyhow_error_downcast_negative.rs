// SAFE: downcast_ref is used instead of downcast().unwrap(), returning Option and avoiding panics on unexpected error types.

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
    if let Some(specific) = err.downcast_ref::<MyError>() {
        println!("handled: {}", specific);
    }
}

fn process_result(res: Result<(), Error>) {
    if let Err(e) = res {
        if let Ok(my_err) = e.downcast::<MyError>() {
            let _ = my_err;
        }
    }
}
