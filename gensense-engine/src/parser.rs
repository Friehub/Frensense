// SPDX-License-Identifier: MIT

use crate::GenSenseError;
use crate::Result;
use std::path::Path;
use tree_sitter::Language;

/// Maps file extensions to tree-sitter language names.
const LANGUAGE_EXTENSIONS: &[(&[&str], &[&str])] = &[
    (&["rust"], &["rs"]),
    (&["typescript", "ts"], &["ts", "tsx"]),
    (&["javascript", "js"], &["js", "jsx"]),
    (&["yaml", "yml"], &["yml", "yaml"]),
];

pub struct ParserRegistry;

impl ParserRegistry {
    /// Returns the tree-sitter language for a given file path.
    pub fn get_language(path: &Path) -> Result<Language> {
        let ext = path.extension().and_then(|s| s.to_str()).ok_or_else(|| {
            GenSenseError::Config(format!("File has no extension: {}", path.display()))
        })?;

        match ext {
            #[cfg(feature = "rust")]
            "rs" => Ok(tree_sitter_rust::LANGUAGE.into()),
            #[cfg(feature = "typescript")]
            "ts" | "tsx" => Ok(tree_sitter_typescript::LANGUAGE_TSX.into()),
            #[cfg(feature = "typescript")]
            "js" | "jsx" => Ok(tree_sitter_javascript::LANGUAGE.into()),
            // YAML tree-sitter parsing not available in engine; YAML is consumer-side via serde_yaml
            "yml" | "yaml" => Err(GenSenseError::Config(format!(
                "YAML tree-sitter parsing not available in the engine (use consumer crate). Extension: {ext}"
            ))),
            _ => Err(GenSenseError::Config(format!(
                "Unsupported file extension or feature not enabled: {ext}"
            ))),
        }
    }

    /// Returns the tree-sitter language for a language name string.
    pub fn get_language_by_name(name: &str) -> Result<Language> {
        match name {
            "rust" => Self::get_language(Path::new("x.rs")),
            "typescript" | "ts" => Self::get_language(Path::new("x.tsx")),
            "javascript" | "js" => Self::get_language(Path::new("x.js")),
            "yaml" | "yml" => Self::get_language(Path::new("x.yaml")),
            _ => Err(GenSenseError::Config(format!(
                "Unsupported language: {name}"
            ))),
        }
    }

    pub fn get_symbol_query_by_ext(&self, ext: &str) -> Option<&'static str> {
        match ext {
            "rs" => Some(
                r"
                (function_item name: (identifier) @name)
                (parameter pattern: (identifier) @name)
                (parameter pattern: (tuple_pattern (identifier) @name))
                (let_declaration pattern: (identifier) @name)
                (let_declaration pattern: (tuple_pattern (identifier) @name))
                (struct_item name: (type_identifier) @name)
                (enum_item name: (type_identifier) @name)
                (trait_item name: (type_identifier) @name)
                (const_item name: (identifier) @name)
            ",
            ),
            "ts" | "tsx" => Some(
                r"
                (function_declaration name: (identifier) @name)
                (method_definition name: (property_identifier) @name)
                (class_declaration name: (type_identifier) @name)
                (interface_declaration name: (type_identifier) @name)
                (enum_declaration name: (identifier) @name)
                (variable_declarator name: (identifier) @name)
                (lexical_declaration (variable_declarator name: (identifier) @name))
            ",
            ),
            "js" | "jsx" => Some(
                r"
                (function_declaration name: (identifier) @name)
                (class_declaration name: (identifier) @name)
                (variable_declarator name: (identifier) @name)
                (lexical_declaration (variable_declarator name: (identifier) @name))
            ",
            ),
            _ => None,
        }
    }

    pub fn get_call_query_by_ext(&self, ext: &str) -> Option<&'static str> {
        match ext {
            "rs" => Some(
                r"
                (call_expression function: (identifier) @call)
                (call_expression function: (field_expression field: (field_identifier) @call))
            ",
            ),
            "ts" | "tsx" | "js" | "jsx" => Some(
                r"
                (call_expression function: (identifier) @call)
                (call_expression function: (member_expression property: (property_identifier) @call))
            ",
            ),
            _ => None,
        }
    }

    pub fn is_supported(path: &Path) -> bool {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        matches!(ext, "rs" | "ts" | "tsx" | "js" | "jsx" | "yml" | "yaml")
    }

    pub fn extensions_for(name: &str) -> Option<&'static [&'static str]> {
        let lower = name.to_lowercase();
        LANGUAGE_EXTENSIONS
            .iter()
            .find(|(names, _)| names.contains(&lower.as_str()))
            .map(|(_, exts)| *exts)
    }

    /// Backward-compatible wrapper: get symbol query by path.
    pub fn get_symbol_query(path: &Path) -> Option<&'static str> {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let registry = ParserRegistry;
        registry.get_symbol_query_by_ext(ext)
    }

    /// Backward-compatible wrapper: get call query by path.
    pub fn get_call_query(path: &Path) -> Option<&'static str> {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let registry = ParserRegistry;
        registry.get_call_query_by_ext(ext)
    }

    pub fn ext_matches(ext: &str, allowed: &[&str]) -> bool {
        allowed.contains(&ext)
    }
}
