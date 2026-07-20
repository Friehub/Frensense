// SAFE: All relevant fields are included in the #[error("...")] format string for complete error messages.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("file not found: {path}")]
    FileNotFound { path: String },

    #[error("request failed with status {status}: {body}")]
    HttpError { status: u16, body: String },
}
