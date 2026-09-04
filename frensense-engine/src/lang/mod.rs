// SPDX-License-Identifier: MIT

pub mod kinds;
pub mod mapper;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    TypeScript,
    JavaScript,
    C,
    Python,
    #[allow(dead_code)]
    Html,
}
