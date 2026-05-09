// [LICENSE] Proprietary - Friehub (TaaS Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use crate::{AuditorError, Result};
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
            .ok_or_else(|| AuditorError::Config(format!("File has no extension: {path:?}")))?;

        match ext {
            "rs" => Ok(tree_sitter_rust::LANGUAGE.into()),
            "ts" | "tsx" => Ok(tree_sitter_typescript::LANGUAGE_TSX.into()),
            "js" | "jsx" => Ok(tree_sitter_javascript::LANGUAGE.into()),
            "sol" => Err(AuditorError::Config(
                "Solidity is temporarily disabled due to tree-sitter version incompatibility."
                    .to_string(),
            )),
            "yml" | "yaml" => Ok(tree_sitter_yaml::LANGUAGE.into()),
            _ => Err(AuditorError::Config(format!(
                "Unsupported file extension: {ext}"
            ))),
        }
    }

    pub fn get_symbol_query(path: &Path) -> Option<&'static str> {
        let ext = path.extension().and_then(|s| s.to_str())?;
        match ext {
            "rs" => Some(
                r#"
                (function_item name: (identifier) @name) @fn
                (struct_item name: (type_identifier) @name) @struct
                (enum_item name: (type_identifier) @name) @enum
                (trait_item name: (type_identifier) @name) @interface
                (const_item name: (identifier) @name) @const
            "#,
            ),
            "ts" | "tsx" => Some(
                r#"
                (function_declaration name: (identifier) @name) @fn
                (class_declaration name: (type_identifier) @name) @class
                (interface_declaration name: (type_identifier) @name) @interface
                (enum_declaration name: (identifier) @name) @enum
                (variable_declarator name: (identifier) @name) @var
                (lexical_declaration (variable_declarator name: (identifier) @name)) @var
            "#,
            ),
            "js" | "jsx" => Some(
                r#"
                (function_declaration name: (identifier) @name) @fn
                (class_declaration name: (identifier) @name) @class
                (variable_declarator name: (identifier) @name) @var
                (lexical_declaration (variable_declarator name: (identifier) @name)) @var
            "#,
            ),
            "sol" => Some(
                r#"
                (contract_declaration name: (identifier) @name) @class
                (interface_declaration name: (identifier) @name) @interface
                (library_declaration name: (identifier) @name) @module
                (function_definition name: (identifier) @name) @fn
                (struct_definition name: (identifier) @name) @struct
                (enum_definition name: (identifier) @name) @enum
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
