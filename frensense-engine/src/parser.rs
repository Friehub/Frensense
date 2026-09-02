// SPDX-License-Identifier: MIT

use crate::FrensenseError;
use crate::Result;
use std::path::Path;
use tree_sitter::Language;

/// Maps file extensions to tree-sitter language names.
const LANGUAGE_EXTENSIONS: &[(&[&str], &[&str])] = &[
    (&["rust"], &["rs"]),
    (&["typescript", "ts"], &["ts", "tsx"]),
    (&["javascript", "js"], &["js", "jsx"]),
    (&["python", "py"], &["py", "pyi"]),
    (&["yaml", "yml"], &["yml", "yaml"]),
    (&["html"], &["html", "htm"]),
];

/// Maps file extension to human-readable language name.
pub fn ext_to_language(ext: &str) -> &'static str {
    match ext {
        "rs" => "rust",
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "py" | "pyi" => "python",
        "yml" | "yaml" => "yaml",
        "html" | "htm" => "html",
        _ => "unknown",
    }
}

/// Check whether a file has a supported extension.
pub fn is_supported(path: &Path) -> bool {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    matches!(
        ext,
        "rs" | "ts" | "tsx" | "js" | "jsx" | "py" | "pyi" | "yml" | "yaml" | "json" | "html" | "htm"
    )
}

/// Look up file extensions for a language name (e.g. `rust` → `["rs"]`).
pub fn extensions_for(name: &str) -> Option<&'static [&'static str]> {
    let lower = name.to_lowercase();
    LANGUAGE_EXTENSIONS
        .iter()
        .find(|(names, _)| names.contains(&lower.as_str()))
        .map(|(_, exts)| *exts)
}

/// Check whether a file extension matches one of the given allowed extensions.
pub fn ext_matches(ext: &str, allowed: &[&str]) -> bool {
    allowed.contains(&ext)
}

/// Tree-sitter symbol query for a file extension.
pub fn symbol_query_for_ext(ext: &str) -> Option<&'static str> {
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
            (method_definition name: (property_identifier) @name)
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
        "html" | "htm" => Some(
            r"
            (element (tag_name) @name)
            (script_element (tag_name) @name)
            (style_element (tag_name) @name)
        ",
        ),
        _ => None,
    }
}

/// Tree-sitter call query for a file extension.
///
/// Each match captures two names:
///   @caller — the enclosing function / method name
///   @call   — the callee being invoked
///
/// This enables `extract_edges_from_tree` to wire `caller → callee` edges
/// in the `SemanticGraph` without a separate AST walk.
pub fn call_query_for_ext(ext: &str) -> Option<&'static str> {
    match ext {
        "rs" => Some(
            r"
            (function_item name: (identifier) @caller
                body: (_
                    (call_expression function: (identifier) @call)))
            (function_item name: (identifier) @caller
                body: (_
                    (call_expression function:
                        (field_expression field: (field_identifier) @call))))
            (function_item name: (identifier) @caller
                body: (_
                    (let_declaration
                        value: (call_expression function: (identifier) @call))))
            (function_item name: (identifier) @caller
                body: (_
                    (let_declaration
                        value: (call_expression function:
                            (field_expression field: (field_identifier) @call)))))
        ",
        ),
        "ts" | "tsx" | "js" | "jsx" => Some(
            r"
            (function_declaration name: (identifier) @caller
                body: (_
                    (expression_statement
                        (call_expression function: (identifier) @call))))
            (function_declaration name: (identifier) @caller
                body: (_
                    (expression_statement
                        (call_expression function:
                            (member_expression property: (property_identifier) @call)))))
            (method_definition name: (property_identifier) @caller
                body: (_
                    (expression_statement
                        (call_expression function: (identifier) @call))))
            (method_definition name: (property_identifier) @caller
                body: (_
                    (expression_statement
                        (call_expression function:
                            (member_expression property: (property_identifier) @call)))))
            (expression_statement
                (assignment_expression
                    left: (member_expression
                        property: (property_identifier) @caller)
                    right: (arrow_function
                        body: (statement_block
                            (expression_statement
                                (call_expression function: (identifier) @call))))))
            (expression_statement
                (assignment_expression
                    left: (member_expression
                        property: (property_identifier) @caller)
                    right: (arrow_function
                        body: (statement_block
                            (expression_statement
                                (call_expression function:
                                    (member_expression property: (property_identifier) @call)))))))
        ",
        ),
        "py" | "pyi" => Some(
            r"
            (function_definition name: (identifier) @caller
                body: (_
                    (expression_statement
                        (call function: (identifier) @call))))
            (function_definition name: (identifier) @caller
                body: (_
                    (expression_statement
                        (call function: (attribute attribute: (identifier) @call)))))
        ",
        ),
        "html" | "htm" => None,
        _ => None,
    }
}

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
            #[cfg(feature = "html")]
            "html" | "htm" => Ok(tree_sitter_html::LANGUAGE.into()),
            "yml" | "yaml" => Err(FrensenseError::Config(format!(
                "YAML tree-sitter parsing not available in the engine (use consumer crate). Extension: {ext}"
            ))),
            _ => Err(FrensenseError::Config(format!(
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
            "python" | "py" => Self::get_language(Path::new("x.py")),
            "yaml" | "yml" => Self::get_language(Path::new("x.yaml")),
            "html" => Self::get_language(Path::new("x.html")),
            _ => Err(FrensenseError::Config(format!(
                "Unsupported language: {name}"
            ))),
        }
    }

    pub fn get_symbol_query_by_ext(&self, ext: &str) -> Option<&'static str> {
        symbol_query_for_ext(ext)
    }

    pub fn get_call_query_by_ext(&self, ext: &str) -> Option<&'static str> {
        call_query_for_ext(ext)
    }

    pub fn is_supported(path: &Path) -> bool {
        crate::parser::is_supported(path)
    }

    pub fn extensions_for(name: &str) -> Option<&'static [&'static str]> {
        crate::parser::extensions_for(name)
    }

    pub fn get_symbol_query(path: &Path) -> Option<&'static str> {
        let ext = path.extension().and_then(|s| s.to_str())?;
        symbol_query_for_ext(ext)
    }

    pub fn get_call_query(path: &Path) -> Option<&'static str> {
        let ext = path.extension().and_then(|s| s.to_str())?;
        call_query_for_ext(ext)
    }

    pub fn ext_matches(ext: &str, allowed: &[&str]) -> bool {
        crate::parser::ext_matches(ext, allowed)
    }
}
