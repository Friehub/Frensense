// SPDX-License-Identifier: MIT

use crate::{FrensenseError, Result};
use std::path::Path;
use tree_sitter::Language;

/// Maps language names (as passed via `--language`) to their file extensions.
/// Adding a new language means adding an entry here and in `get_language`/`is_supported`.
const LANGUAGE_EXTENSIONS: &[(&[&str], &[&str])] = &[
    (&["rust"], &["rs"]),
    (&["typescript", "ts"], &["ts", "tsx"]),
    (&["javascript", "js"], &["js", "jsx"]),
    (&["python", "py"], &["py", "pyi"]),
    (&["yaml", "yml"], &["yml", "yaml"]),
];

pub struct ParserRegistry;

impl ParserRegistry {
    /// Returns the tree-sitter language for a given file path.
    ///
    /// # Errors
    /// Returns an error if the file extension is missing or if the language is not supported.
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
            "py" | "pyi" => Some(
                r"
                (function_definition name: (identifier) @name)
                (class_definition name: (identifier) @name)
                (assignment left: (identifier) @name)
            ",
            ),
            _ => None,
        }
    }

    #[must_use]
    pub fn get_call_query(path: &Path) -> Option<&'static str> {
        let ext = path.extension().and_then(|s| s.to_str())?;
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
            "py" | "pyi" => Some(
                r"
                (call function: (identifier) @call)
                (call function: (attribute attribute: (identifier) @call))
            ",
            ),
            _ => None,
        }
    }

    #[must_use]
    pub fn is_supported(path: &Path) -> bool {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        matches!(
            ext,
            "rs" | "ts" | "tsx" | "js" | "jsx" | "py" | "pyi" | "yml" | "yaml"
        )
    }

    /// Look up file extensions for a language name as passed via `--language`.
    /// Returns `None` if the name is not recognised or if the language feature is disabled.
    #[must_use]
    pub fn extensions_for(name: &str) -> Option<&'static [&'static str]> {
        let lower = name.to_lowercase();
        LANGUAGE_EXTENSIONS
            .iter()
            .find(|(names, _)| names.contains(&lower.as_str()))
            .map(|(_, exts)| *exts)
    }

    /// Check whether a file extension matches one of the given allowed extensions.
    /// Convenience for rule `applies_to()` implementations — keeps the extension list
    /// in one place when a new language is added.
    #[must_use]
    pub fn ext_matches(ext: &str, allowed: &[&str]) -> bool {
        allowed.contains(&ext)
    }
}
