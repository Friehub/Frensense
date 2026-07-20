// [frensense]
// observation: A thiserror #[error("...")] annotation omits important fields from the display message, hiding context from error messages.
// impact: When the error is displayed to users or logged, critical context (e.g., file path, HTTP status code, invalid value) is missing, making debugging and error reporting ineffective.
// improvement: Include all relevant fields in the #[error("...")] format string using their Display or Debug implementations.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("file not found")]
    FileNotFound { path: String },

    #[error("request failed")]
    HttpError { status: u16, body: String },
}

fn show_error(err: &AppError) {
    eprintln!("{}", err);
}
