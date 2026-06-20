// SPDX-License-Identifier: MIT

use crate::{FrensenseError, Result};
use std::path::Path;
use tree_sitter::Language;

pub struct ParserRegistry;

impl ParserRegistry {
    /// Returns the tree-sitter language for a given file path.
    pub fn get_language(path: &Path) -> Result<Language> {
        let ext = path.extension().and_then(|s| s.to_str()).ok_or_else(|| {
            FrensenseError::Config(format!("File has no extension: {}", path.display()))
        })?;

        match ext {
            #[cfg(feature = "rust")]
            "rs" => Ok(tree_sitter_rust::LANGUAGE.into()),
            #[cfg(feature = "typescript")]
            "ts" | "tsx" => Ok(tree_sitter_typescript::LANGUAGE_TSX.into()),
            #[cfg(feature = "typescript")]
            "js" | "jsx" => Ok(tree_sitter_javascript::LANGUAGE.into()),
            #[cfg(feature = "python")]
            "py" | "pyi" => Ok(tree_sitter_python::LANGUAGE.into()),
            #[cfg(feature = "c_lang")]
            "c" | "h" => Ok(tree_sitter_c::LANGUAGE.into()),
            "yml" | "yaml" => Err(FrensenseError::Config(
                "YAML tree-sitter parsing not available in this build".to_string(),
            )),
            _ => Err(FrensenseError::Config(format!(
                "Unsupported file extension or feature not enabled: {ext}"
            ))),
        }
    }

    #[must_use]
    pub fn get_symbol_query(path: &Path) -> Option<&'static str> {
        let ext = path.extension().and_then(|s| s.to_str())?;
        frensense_engine::parser::symbol_query_for_ext(ext)
    }

    #[must_use]
    pub fn get_call_query(path: &Path) -> Option<&'static str> {
        let ext = path.extension().and_then(|s| s.to_str())?;
        frensense_engine::parser::call_query_for_ext(ext)
    }

    #[must_use]
    pub fn is_supported(path: &Path) -> bool {
        frensense_engine::parser::is_supported(path)
    }

    #[must_use]
    pub fn extensions_for(name: &str) -> Option<&'static [&'static str]> {
        frensense_engine::parser::extensions_for(name)
    }

    #[must_use]
    pub fn ext_matches(ext: &str, allowed: &[&str]) -> bool {
        frensense_engine::parser::ext_matches(ext, allowed)
    }
}
