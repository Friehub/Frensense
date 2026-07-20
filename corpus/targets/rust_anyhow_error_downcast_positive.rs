// [frensense]
// observation: An anyhow::Error is downcast to a specific error type without checking the result, causing a panic if the error is of a different variant.
// impact: If the inner error is not the expected type, error.downcast::<MyError>() unwrap causes a panic, turning a recoverable error into a crash.
// improvement: Use error.downcast_ref::<T>() which returns Option<&T>, or handle the downcast error case explicitly.

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
    let specific = err.downcast::<MyError>().unwrap();
    println!("handled: {}", specific);
}

fn process_result(res: Result<(), Error>) {
    if let Err(e) = res {
        let _my_err: MyError = *e.downcast::<MyError>().unwrap();
    }
}
