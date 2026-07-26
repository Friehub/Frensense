// SPDX-License-Identifier: MIT

//! Function role classifier — identifies what a function DOES from its fingerprint.
//!
//! A lightweight structural classifier that assigns one of 5 roles with zero
//! corpus lookup.  Used as a pre-filter before scoring: if the candidate's role
//! is incompatible with the pattern's role, the pattern can't possibly match.

use crate::fingerprint::FunctionFingerprint;

/// High-level role a function plays in the codebase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionRole {
    /// Express/Fastify/Hono route handler: params include req/res, calls res.*
    HttpHandler,
    /// Reads or writes a database: calls query/execute/prepare/raw
    DbQuery,
    /// Spawns or executes a system command: calls exec/spawn/system
    ShellExecutor,
    /// Pure data transformation: no control flow, no API calls, just ngrams
    DataTransformer,
    /// None of the above
    Unknown,
}

/// Known Express/HTTP response method names (last-segment form) across languages.
const HTTP_METHODS: &[&str] = &[
    // JS/TS
    "json", "send", "redirect", "status", "render", "end", "write", "setheader", "cookie", "clearcookie", "type", "format", "attachment",
    // Go
    "writeheader", "setcookie", "writestring", "servehttp",
    // Rust
    "ok", "created", "internal_server_error", "into_response", "content_type", "body", "finish",
    // Java/C#
    "badrequest", "notfound", "addcookie", "setstatus", "view"
];

/// Known HTTP request property names (used as function calls or access patterns).
const HTTP_REQUEST: &[&str] = &["req", "res", "next", "request", "response"];

/// Known database query API names across languages.
const DB_API: &[&str] = &[
    // Generic / JS
    "query", "execute", "prepare", "raw", "find", "findone", "findmany", "insert", "update", "delete", "create", "save", "select", "from", "where", "join", "aggregate", "count", "transaction", "commit", "rollback", "upsert",
    // Go
    "queryrow", "exec", "begin", "first", "updates",
    // Rust
    "fetch_one", "fetch_optional", "fetch_all", "load", "get_result", "insert_into",
    // Java/C#
    "persist", "merge", "remove", "savechanges", "add"
];

/// Known shell execution API names across languages.
const SHELL_API: &[&str] = &[
    // Generic / JS
    "exec", "spawn", "execfile", "execsync", "spawnsync", "system", "popen", "run", "cmd", "sh", "bash",
    // Go
    "command", "output", "combinedoutput", "start",
    // Rust
    "command::new", "status",
    // Java/C#
    "getruntime().exec", "processbuilder", "process.start"
];

/// Classify a function's role from its fingerprint.
///
/// Uses only the fingerprint data (no AST access needed):
/// - `api_calls` / `api_call_segments` for method-level detection
/// - `signature_ngrams` / `param_type_ngrams` for parameter shape
///
/// The checks are ordered by priority — HttpHandler is checked first
/// because its signal (res.json/send/redirect) is the strongest.
pub fn classify_role(fp: &FunctionFingerprint) -> FunctionRole {
    // Use raw_call_names to directly inspect what APIs are being called.
    let _all_calls = &fp.raw_call_names;

    // --- HttpHandler detection ---
    // Looks for `res.xxx` call patterns via structural markers + control flow.
    // An HTTP handler typically has:  params(a, b, c)  +  res.something()  +  if/return
    // We detect this through:
    //  1. param_types containing "Request" or "Response"
    //  2. api_call_segments from res.json/redirect/send/status
    //  3. signature n-grams containing "req" or "res"
    if is_http_handler(fp) {
        return FunctionRole::HttpHandler;
    }

    // --- ShellExecutor detection ---
    if is_shell_executor(fp) {
        return FunctionRole::ShellExecutor;
    }

    // --- DbQuery detection ---
    if is_db_query(fp) {
        return FunctionRole::DbQuery;
    }

    // --- DataTransformer detection ---
    if fp.control_flow_hashes.is_empty()
        && fp.api_calls.is_empty()
        && fp.api_call_segments.is_empty()
    {
        return FunctionRole::DataTransformer;
    }

    FunctionRole::Unknown
}

/// Check if fingerprint matches an HTTP request/response handler.
fn is_http_handler(fp: &FunctionFingerprint) -> bool {
    let has_http_call = fp.raw_call_names.iter().any(|c| {
        let lower = c.to_lowercase();
        HTTP_METHODS.iter().any(|m| lower.ends_with(m))
    });

    let struct_count = fp.structural_markers.len();
    let has_api = !fp.api_calls.is_empty();
    let has_control_flow = !fp.control_flow_hashes.is_empty();
    let type_count = fp.param_type_ngrams.len();

    has_http_call || (struct_count >= 7 && has_api && type_count >= 2 && has_control_flow)
}

/// Check if fingerprint matches a shell executor.
fn is_shell_executor(fp: &FunctionFingerprint) -> bool {
    let has_shell_call = fp.raw_call_names.iter().any(|c| {
        let lower = c.to_lowercase();
        SHELL_API.iter().any(|api| lower.ends_with(api))
    });

    let has_api = !fp.api_calls.is_empty();
    let has_control = !fp.control_flow_hashes.is_empty();
    let signature_count = fp.signature_ngrams.len();
    let type_count = fp.param_type_ngrams.len();

    has_shell_call || (has_api && has_control && type_count <= 2 && signature_count <= 3)
}

/// Check if fingerprint matches a database query function.
fn is_db_query(fp: &FunctionFingerprint) -> bool {
    let has_db_call = fp.raw_call_names.iter().any(|c| {
        let lower = c.to_lowercase();
        DB_API.iter().any(|api| lower.ends_with(api))
    });

    let has_api = !fp.api_calls.is_empty();
    let has_control = !fp.control_flow_hashes.is_empty();
    let type_count = fp.param_type_ngrams.len();

    has_db_call || (has_api && has_control && type_count >= 1)
}

/// Check if two roles are incompatible (cannot be the same function).
///
/// - An HttpHandler can NEVER be a ShellExecutor or DbQuery
/// - A DataTransformer is compatible with everything (too generic)
/// - Unknown is compatible with everything (no information)
pub fn roles_are_incompatible(role_a: FunctionRole, role_b: FunctionRole) -> bool {
    use FunctionRole::*;
    match (role_a, role_b) {
        (HttpHandler, ShellExecutor) | (ShellExecutor, HttpHandler) => true,
        (HttpHandler, DbQuery) | (DbQuery, HttpHandler) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_fp(
        api_calls: Vec<u64>,
        api_call_segments: Vec<u64>,
        control_flow: Vec<u64>,
        structural: Vec<u64>,
        sig: Vec<u64>,
        param_types: Vec<u64>,
    ) -> FunctionFingerprint {
        FunctionFingerprint {
            file_path: String::new(),
            function_name: String::new(),
            line: 0,
            language: String::new(),
            ngram_hashes: Vec::new(),
            weighted_ngram_hashes: Default::default(),
            signature_ngrams: sig,
            param_type_ngrams: param_types,
            name_segments: Vec::new(),
            structural_markers: structural,
            type_usages: Vec::new(),
            comment_density: 0.0,
            semantic_markers: Vec::new(),
            skeleton: Vec::new(),
            skeleton_hashes: Vec::new(),
            control_flow_hashes: control_flow,
            api_calls,
            api_call_segments,
            property_accesses: Vec::new(),
            motif_hashes: Vec::new(),
            data_flow_path_hashes: Vec::new(),
            raw_call_names: Vec::new(),
            tainted_api_calls: Vec::new(),
            config_literal_hashes: Vec::new(),
        }
    }

    #[test]
    fn test_http_handler_classification() {
        // HttpHandler: lots of structural markers + API calls + control flow
        let fp = make_fp(
            vec![1, 2, 3],                    // api_calls
            vec![4, 5],                       // segments
            vec![10, 11],                     // control_flow
            vec![20, 21, 22, 23, 24, 25, 26], // structural (7+)
            vec![30, 31],                     // sig
            vec![40, 41],                     // param_types
        );
        assert_eq!(classify_role(&fp), FunctionRole::HttpHandler);
    }

    #[test]
    fn test_shell_executor_classification() {
        // ShellExecutor: few API calls, few param types, some control flow
        // Must NOT match HttpHandler (needs >= 3 api_calls and >= 2 types)
        let fp = make_fp(
            vec![1, 2], // only 2 api_calls
            vec![],
            vec![10],                     // some control flow
            vec![20, 21, 22, 23, 24, 25], // 6 structural
            vec![30],
            vec![40], // only 1 param type
        );
        assert_eq!(classify_role(&fp), FunctionRole::ShellExecutor);
    }

    #[test]
    fn test_data_transformer_classification() {
        // DataTransformer: no API calls, no control flow
        let fp = make_fp(vec![], vec![], vec![], vec![20, 21, 22], vec![30], vec![]);
        assert_eq!(classify_role(&fp), FunctionRole::DataTransformer);
    }

    #[test]
    fn test_roles_incompatible() {
        assert!(roles_are_incompatible(
            FunctionRole::HttpHandler,
            FunctionRole::ShellExecutor
        ));
        assert!(roles_are_incompatible(
            FunctionRole::ShellExecutor,
            FunctionRole::HttpHandler
        ));
        assert!(roles_are_incompatible(
            FunctionRole::HttpHandler,
            FunctionRole::DbQuery
        ));
        assert!(!roles_are_incompatible(
            FunctionRole::HttpHandler,
            FunctionRole::HttpHandler
        ));
        assert!(!roles_are_incompatible(
            FunctionRole::Unknown,
            FunctionRole::ShellExecutor
        ));
    }
}
