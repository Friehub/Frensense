// SPDX-License-Identifier: MIT

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct FunctionFingerprint {
    pub file_path: String,
    pub function_name: String,
    pub line: usize,
    pub language: String,
    pub name_segments: Vec<String>,
    pub type_usages: Vec<String>,
    pub comment_density: f64,
    pub semantic_markers: Vec<u64>,
    pub skeleton: Vec<String>,
    #[cfg_attr(feature = "serialize", serde(default))]
    pub skeleton_hashes: Vec<u64>,
    /// Control flow encoding: hashes of control flow paths through the function.
    pub control_flow_hashes: Vec<u64>,
    /// Hash of the ordered control-flow event sequence.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub control_flow_sequence: Vec<u64>,
    /// API calls: hashes of the full callee expression.
    pub api_calls: Vec<u64>,
    /// Last-segment hashes of chained method calls.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub api_call_segments: Vec<u64>,
    /// Property accesses: hashes of object property access names.
    pub property_accesses: Vec<u64>,
    /// Motif hashes: canonical motif name hash for each matching API call.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub motif_hashes: Vec<u64>,
    /// Data-flow path hashes: abstract source→sink chains.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub data_flow_path_hashes: Vec<u64>,
    /// Raw call target name strings.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub raw_call_names: Vec<String>,
    /// Parameter names from the function signature.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub param_names: Vec<String>,
    /// API calls where at least one argument is a function parameter.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub tainted_api_calls: Vec<u64>,
    /// Hashes of (function_segment, arg_position, arg_ast_kind) per call.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub argument_call_types: Vec<u64>,
    /// Hashes of content patterns in string/template literal call arguments.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub literal_pattern_hashes: Vec<u64>,
    /// Whether this function has a routing decorator (e.g. `@Get`, `@Post`).
    #[cfg_attr(feature = "serialize", serde(default))]
    pub has_http_decorator: bool,
    /// Whether this function is referenced as a handler in a route registration.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub is_registered_handler: bool,
    /// Export handler kind from file-level export patterns.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub export_handler_kind: Option<crate::export_matcher::ExportHandlerKind>,
    /// Whether any assigned variable was confirmed to originate from a known source
    #[cfg_attr(feature = "serialize", serde(default))]
    pub has_confirmed_source: bool,
}
