// SPDX-License-Identifier: MIT

use frensense_lang::NodeRole;

use crate::lang::Language;
use crate::lang::kinds::AbstractKind;

/// Map a tree-sitter node kind to an [`AbstractKind`] using the
/// [`frensense_lang`] registry as the single source of truth.
///
/// Previously this function contained six separate per-language match arms
/// (Rust, TypeScript, JavaScript, C, Python, Go). Now it delegates to
/// `Language::spec().classify()` and bridges `NodeRole` → `AbstractKind`.
pub fn abstract_kind(ts_kind: &str, language: Language) -> AbstractKind {
    // If we have a spec for this language, use it.
    if let Some(spec) = language.spec() {
        return node_role_to_abstract_kind(spec.classify(ts_kind), ts_kind, language);
    }

    // Html has no spec registered; keep a minimal inline map.
    match ts_kind {
        "element" | "script_element" | "style_element" => AbstractKind::Block,
        "text" | "doctype" => AbstractKind::StringLiteral,
        _ => AbstractKind::Other,
    }
}

/// Convert a [`NodeRole`] (from frensense-lang) into the engine's
/// [`AbstractKind`] (used for structural hashing in the fingerprinter).
///
/// `NodeRole` is the canonical, language-agnostic classification.
/// `AbstractKind` is a slightly different enumeration that the engine has
/// been using for structural n-grams. This bridge keeps both working while
/// the engine migrates incrementally.
fn node_role_to_abstract_kind(role: NodeRole, ts_kind: &str, language: Language) -> AbstractKind {
    match role {
        // ── Definitions ──────────────────────────────────────────────────
        NodeRole::Function {
            is_method: true, ..
        } => AbstractKind::MethodDef,
        NodeRole::Function {
            is_method: false, ..
        } => {
            // Closures and lambdas still want Closure in the abstract kind
            let is_closure = matches!(
                ts_kind,
                "arrow_function"
                    | "closure_expression"
                    | "func_literal"
                    | "lambda"
                    | "function_expression"
                    | "async_function_expression"
            );
            if is_closure {
                AbstractKind::Closure
            } else {
                AbstractKind::FunctionDef
            }
        }

        // ── Declarations / assignments ───────────────────────────────────
        NodeRole::Declaration { .. } => AbstractKind::Assign,
        NodeRole::Assignment { .. } => AbstractKind::Assign,

        // ── Calls ────────────────────────────────────────────────────────
        NodeRole::Call { .. } => AbstractKind::Call,
        NodeRole::MemberAccess { .. } => AbstractKind::MethodCall,

        // ── Control flow ─────────────────────────────────────────────────
        NodeRole::Branch => AbstractKind::Conditional,
        NodeRole::Loop => AbstractKind::Loop,
        NodeRole::Return => AbstractKind::Return,
        NodeRole::Try | NodeRole::Catch | NodeRole::Finally => AbstractKind::TryCatch,
        NodeRole::Throw => AbstractKind::Throw,
        NodeRole::ErrorGuard => AbstractKind::Conditional,
        NodeRole::ErrorPropagation => AbstractKind::TryCatch,
        NodeRole::ContextManager => AbstractKind::TryCatch,
        NodeRole::Await => AbstractKind::Await,

        // ── Structural ───────────────────────────────────────────────────
        NodeRole::Block => AbstractKind::Block,
        NodeRole::Import => AbstractKind::ImportDecl,
        NodeRole::Export => AbstractKind::ExportDecl,
        NodeRole::Identifier => AbstractKind::Identifier,
        NodeRole::Literal => {
            // Distinguish string vs number vs bool using the raw ts_kind.
            // This preserves the fingerprint granularity that existing corpus
            // entries were built with.
            match ts_kind {
                "string"
                | "string_literal"
                | "raw_string_literal"
                | "template_string"
                | "interpreted_string_literal"
                | "string_fragment"
                | "char_literal" => AbstractKind::StringLiteral,

                "number" | "integer_literal" | "float_literal" | "int_literal"
                | "number_literal" => AbstractKind::NumberLiteral,

                "true" | "false" | "boolean_literal" | "none" => AbstractKind::BoolLiteral,

                _ => AbstractKind::StringLiteral,
            }
        }

        // ── Supplementary structural roles ───────────────────────────────
        NodeRole::Parameters => AbstractKind::Parameters,
        NodeRole::Arguments => AbstractKind::Arguments,
        NodeRole::ClassDef => AbstractKind::ClassDef,
        NodeRole::BinaryOp => AbstractKind::BinaryOp,
        NodeRole::UnaryOp => AbstractKind::UnaryOp,
        NodeRole::Match => AbstractKind::Match,
        NodeRole::Unsafe => AbstractKind::Unsafe,
        NodeRole::AsyncBlock => AbstractKind::Async,

        // ── Language-specific extras via raw ts_kind ─────────────────────
        // NodeRole::Other covers things the lang spec doesn't classify.
        // We still want to catch a few engine-specific extras per language.
        NodeRole::Other => other_to_abstract_kind(ts_kind, language),
    }
}

/// Fallback handler for `NodeRole::Other` - maps a small set of language-
/// specific node kinds that `AbstractKind` tracks but `NodeRole` doesn't have
/// a variant for (e.g. struct/enum/trait definitions that use `Other`).
fn other_to_abstract_kind(ts_kind: &str, language: Language) -> AbstractKind {
    match language {
        Language::Rust => match ts_kind {
            "const_item" => AbstractKind::ConstDef,
            "mod_item" => AbstractKind::ModuleDef,
            "type_identifier"
            | "primitive_type"
            | "scoped_identifier"
            | "scoped_type_identifier" => AbstractKind::Identifier,
            _ => AbstractKind::Other,
        },
        Language::TypeScript | Language::JavaScript => match ts_kind {
            "type_annotation" | "type_arguments" => AbstractKind::Other,
            _ => AbstractKind::Other,
        },
        Language::Python => match ts_kind {
            "type" => AbstractKind::TypeAnnotation,
            _ => AbstractKind::Other,
        },
        Language::Go => match ts_kind {
            "defer_statement" | "go_statement" => AbstractKind::Call,
            "field_identifier" | "type_identifier" => AbstractKind::Identifier,
            _ => AbstractKind::Other,
        },
        Language::C => AbstractKind::Other,
        Language::Html => AbstractKind::Other,
    }
}
