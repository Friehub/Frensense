// SPDX-License-Identifier: MIT

use crate::{GenSenseError, Result};
use std::path::Path;
use tree_sitter::Language;

pub enum SupportedLanguage {
    Rust,
    TypeScript,
    JavaScript,
    Solidity,
    Yaml,
}

pub struct ParserRegistry;

impl ParserRegistry {
    pub fn get_language(path: &Path) -> Result<Language> {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| GenSenseError::Config(format!("File has no extension: {path:?}")))?;

        match ext {
            #[cfg(feature = "rust")]
            "rs" => Ok(tree_sitter_rust::LANGUAGE.into()),
            #[cfg(feature = "typescript")]
            "ts" | "tsx" => Ok(tree_sitter_typescript::LANGUAGE_TSX.into()),
            #[cfg(feature = "typescript")]
            "js" | "jsx" => Ok(tree_sitter_javascript::LANGUAGE.into()),
            #[cfg(feature = "solidity")]
            // "sol" => Ok(tree_sitter_solidity::language().into()),
            "sol" => Err(GenSenseError::Config(
                "Solidity parser is temporarily disabled due to version mismatch".to_string(),
            )),
            "yml" | "yaml" => Ok(tree_sitter_yaml::LANGUAGE.into()),
            _ => Err(GenSenseError::Config(format!(
                "Unsupported file extension or feature not enabled: {ext}"
            ))),
        }
    }

    pub fn get_symbol_query(path: &Path) -> Option<&'static str> {
        let ext = path.extension().and_then(|s| s.to_str())?;
        match ext {
            "rs" => Some(
                r#"
                (function_item name: (identifier) @name)
                (parameter pattern: (identifier) @name)
                (let_declaration pattern: (identifier) @name)
                (struct_item name: (type_identifier) @name)
                (enum_item name: (type_identifier) @name)
                (trait_item name: (type_identifier) @name)
                (const_item name: (identifier) @name)
            "#,
            ),
            "ts" | "tsx" => Some(
                r#"
                (function_declaration name: (identifier) @name)
                (class_declaration name: (type_identifier) @name)
                (interface_declaration name: (type_identifier) @name)
                (enum_declaration name: (identifier) @name)
                (variable_declarator name: (identifier) @name)
                (lexical_declaration (variable_declarator name: (identifier) @name))
            "#,
            ),
            "js" | "jsx" => Some(
                r#"
                (function_declaration name: (identifier) @name)
                (class_declaration name: (identifier) @name)
                (variable_declarator name: (identifier) @name)
                (lexical_declaration (variable_declarator name: (identifier) @name))
            "#,
            ),
            "sol" => Some(
                r#"
                (contract_declaration name: (identifier) @name)
                (interface_declaration name: (identifier) @name)
                (library_declaration name: (identifier) @name)
                (function_definition name: (identifier) @name)
                (struct_definition name: (identifier) @name)
                (enum_definition name: (identifier) @name)
            "#,
            ),
            _ => None,
        }
    }

    pub fn get_call_query(path: &Path) -> Option<&'static str> {
        let ext = path.extension().and_then(|s| s.to_str())?;
        match ext {
            "rs" => Some(
                r#"
                (call_expression function: (identifier) @call)
                (call_expression function: (field_expression field: (field_identifier) @call))
            "#,
            ),
            "ts" | "tsx" | "js" | "jsx" => Some(
                r#"
                (call_expression function: (identifier) @call)
                (call_expression function: (member_expression property: (property_identifier) @call))
            "#,
            ),
            "sol" => Some(
                r#"
                (function_call (identifier) @call)
            "#,
            ),
            _ => None,
        }
    }

    pub fn is_supported(path: &Path) -> bool {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        matches!(
            ext,
            "rs" | "ts" | "tsx" | "js" | "jsx" | "sol" | "yml" | "yaml"
        )
    }
}
