// SPDX-License-Identifier: MIT

//! Corpus-driven source/sink registry.
//!
//! Extracts source types and sink function names from positive corpus files
//! at load time. Replaces hardcoded framework type arrays and sink lists.

use crate::data_flow::TaintOrigin;
use rustc_hash::FxHashMap;
use std::path::Path;
use tree_sitter::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SinkTier {
    HighConfidence, // min_occurrences: 1 — known dangerous, no FP risk
    Standard,       // min_occurrences: 2 — current default
    Suspicious,     // min_occurrences: 3 — novel patterns, need more evidence
}

impl SinkTier {
    pub fn min_occurrences(&self) -> usize {
        match self {
            Self::HighConfidence => 1,
            Self::Standard => 2,
            Self::Suspicious => 3,
        }
    }
}

// Hardcoded high-confidence sinks that are ALWAYS registered regardless of occurrence:
pub const ALWAYS_REGISTER_SINKS: &[&str] = &[
    // Code Execution
    "eval",
    "exec",
    "execSync",
    "spawn",
    "spawnSync",
    "Function",
    "setTimeout",
    "setInterval",
    "runInNewContext",
    "runInThisContext",
    "require",
    "import",
    "Command::new",
    "args",
    // SQL Injection
    "query",
    "execute",
    "executeRaw",
    "queryRaw",
    "raw",
    "sql_query",
    "prepare",
    // Command Injection
    "execFile",
    "execFileSync",
    "shelljs.exec",
    "execa",
    // Path Traversal
    "readFile",
    "writeFile",
    "readFileSync",
    "join",
    "unlink",
    "stat",
    "access",
    "read_to_string",
    "read",
    "write",
    "open",
    // SSRF
    "fetch",
    "axios.get",
    "axios.post",
    "http.get",
    "https.get",
    "got",
    "request",
    "node-fetch",
    "reqwest::get",
    "Client::get",
    "Uri::from",
    "ureq::get",
    // Open Redirect
    "redirect",
    "location.href",
    "window.location",
    // XSS
    "innerHTML",
    "outerHTML",
    "document.write",
    "document.writeln",
    "dangerouslySetInnerHTML",
    // Storage Write
    "put",
    "setItem",
    // Log Leak
    "log",
    "error",
    "info",
    "debug",
    // Unsafe Memory (Rust)
    "transmute",
    "transmute_copy",
    "from_utf8_unchecked",
    "from_raw_parts",
    // Framework Specific (Cloudflare, Express, Next.js, Hono, Prisma)
    "c.redirect",
    "env.KV.put",
    "KVNamespace.put",
    "KVNamespace.delete",
    "env.DB.prepare",
    "res.send",
    "res.json",
    "res.redirect",
    "res.render",
    "revalidatePath",
    "prisma.queryRawUnsafe",
    "prisma.executeRawUnsafe",
    "R2Bucket.put",
    "D1Database.prepare",
    "DurableObjectStub.fetch",
    "Queue.send",
    // SSTI — Template engine renders
    "ejs.render",
    "ejs.renderFile",
    "pug.compile",
    "pug.render",
    "handlebars.compile",
    "handlebars.render",
    "nunjucks.render",
    "nunjucks.renderString",
    "nunjucks.renderFile",
    "marko.render",
    "eta.render",
    "swig.render",
    "liquid.render",
    "mustache.render",
    "jade.render",
    "react-dom/server.renderToString",
    "vue-server-renderer.renderToString",
    "render_template",
    "render_template_string",
    // Insecure Deserialization
    "yaml.load",
    "js-yaml.load",
    "pickle.loads",
    "bincode::deserialize",
    "msgpack.decode",
    "msgpack.unpack",
    "php.unserialize",
    "ObjectInputStream.readObject",
    "BinaryFormatter.Deserialize",
    // Prototype Pollution
    "Object.assign",
    "_.merge",
    "lodash.merge",
    "_.defaultsDeep",
    "_.set",
    "$.extend",
    "jQuery.extend",
    "angular.merge",
    "setPrototypeOf",
    // XXE — XML parsers
    "DOMParser",
    "libxml2",
    "SAXParser",
    "XMLReader",
    "DocumentBuilder",
    "DocumentBuilderFactory",
    "XmlDocument",
    "XDocument",
    "XmlTextReader",
    "simplexml_load_string",
    "DOMDocument",
    // JWT
    "jwt.verify",
    "jwt.decode",
    "jwt.sign",
    "jsonwebtoken.verify",
    "jsonwebtoken.decode",
    "jsonwebtoken.sign",
    "JWT.verify",
    "JWT.decode",
];

pub fn get_sink_tier(sink: &str) -> SinkTier {
    let base = sink.rsplit('.').next().unwrap_or(sink);
    if ALWAYS_REGISTER_SINKS.contains(&base) {
        SinkTier::HighConfidence
    } else {
        SinkTier::Standard
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SinkCategory {
    CodeExecution,
    SqlInjection,
    NoSqlInjection,
    CommandInjection,
    PathTraversal,
    Ssrf,
    OpenRedirect,
    Xss,
    StorageWrite,
    LogLeak,
    ResponseLeak,
    CredentialLeak,
    Unknown,
}

impl SinkCategory {
    pub fn from_sink_name(sink: &str) -> Self {
        let s = sink.to_lowercase();
        if s.contains("eval") || s.contains("exec") && !s.contains("execsync") {
            Self::CodeExecution
        } else if s.contains("query") || s.contains("execute") {
            Self::SqlInjection
        } else if s.contains("spawn") || s.contains("system") || s.contains("execsync") {
            Self::CommandInjection
        } else if s.contains("readfile") || s.contains("writefile") || s.contains("join") {
            Self::PathTraversal
        } else if s.contains("fetch") || s.contains("axios") || s.contains("http.get") {
            Self::Ssrf
        } else if s.contains("redirect") {
            Self::OpenRedirect
        } else if s.contains("innerhtml") || s.contains("document.write") {
            Self::Xss
        } else if s.contains("put") || s.contains("set") {
            Self::StorageWrite
        } else if s.contains("log") || s.contains("logger") {
            Self::LogLeak
        } else if s.contains("send") || s.contains("json") {
            Self::ResponseLeak
        } else {
            Self::Unknown
        }
    }
}

/// Relevance multiplier: how likely a given TaintOrigin is to be a true positive
/// for a given SinkCategory. 1.0 = fully relevant, 0.0 = irrelevant.
/// This prevents false positives where e.g. FileSystem-origin data reaching an
/// SQL sink gets scored the same as UserInput-origin data.
#[must_use]
pub fn sink_taint_relevance(category: SinkCategory, origin: &TaintOrigin) -> f64 {
    match origin {
        TaintOrigin::UserInput => 1.0,
        TaintOrigin::Environment => match category {
            SinkCategory::CredentialLeak | SinkCategory::LogLeak => 0.9,
            _ => 0.5,
        },
        TaintOrigin::Database => match category {
            SinkCategory::SqlInjection | SinkCategory::NoSqlInjection => 0.7,
            SinkCategory::CodeExecution | SinkCategory::CommandInjection => 0.3,
            _ => 0.4,
        },
        TaintOrigin::Network => match category {
            SinkCategory::Ssrf | SinkCategory::OpenRedirect => 0.9,
            SinkCategory::CodeExecution | SinkCategory::CommandInjection => 0.6,
            _ => 0.5,
        },
        TaintOrigin::FileSystem => match category {
            SinkCategory::PathTraversal | SinkCategory::StorageWrite => 0.9,
            SinkCategory::LogLeak => 0.6,
            SinkCategory::SqlInjection | SinkCategory::CodeExecution => 0.2,
            _ => 0.3,
        },
        TaintOrigin::Custom(_) => 1.0,
    }
}

/// Infer the likely SinkCategory from a pattern ID string.
/// Pattern IDs follow `{lang}_{category}_{name}` convention — the name segment
/// often contains keywords like "sql", "cmd", "xss", etc.
#[must_use]
pub fn infer_sink_category(pattern_id: &str) -> Option<SinkCategory> {
    let lower = pattern_id.to_lowercase();
    if lower.contains("sql") || lower.contains("nosql") || lower.contains("sqli") {
        Some(SinkCategory::SqlInjection)
    } else if lower.contains("cmd") || lower.contains("command") || lower.contains("cmdi") {
        Some(SinkCategory::CommandInjection)
    } else if lower.contains("xss") || lower.contains("html_injection") {
        Some(SinkCategory::Xss)
    } else if lower.contains("ssrf") || lower.contains("server_side_request") {
        Some(SinkCategory::Ssrf)
    } else if lower.contains("path_traversal") || lower.contains("read_file") || lower.contains("write_file") {
        Some(SinkCategory::PathTraversal)
    } else if lower.contains("open_redirect") || lower.contains("redirect") {
        Some(SinkCategory::OpenRedirect)
    } else if lower.contains("eval") || lower.contains("code_exec") || lower.contains("rce") {
        Some(SinkCategory::CodeExecution)
    } else if lower.contains("credential") || lower.contains("secret") || lower.contains("password") || lower.contains("token") {
        Some(SinkCategory::CredentialLeak)
    } else if lower.contains("log") {
        Some(SinkCategory::LogLeak)
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub struct CorpusSourceSinkRegistry {
    pub source_types: FxHashMap<String, usize>,
    /// Short method names learned from positive corpus (e.g. `"findOne"`, `"exec"`).
    pub sink_names: FxHashMap<String, (SinkCategory, usize)>,
    /// Qualified call expressions learned from positive corpus (e.g. `"collection.findOne"`).
    /// Used for suffix-match disambiguation to avoid matching generic method names on safe objects.
    pub qualified_sink_names: FxHashMap<String, (SinkCategory, usize)>,
    /// Call expressions that sanitize / escape tainted data, learned from negative corpus.
    pub sanitizer_names: FxHashMap<String, usize>,
}

impl Default for CorpusSourceSinkRegistry {
    fn default() -> Self {
        let mut sink_names = FxHashMap::default();
        let mut qualified_sink_names = FxHashMap::default();

        for &sink in ALWAYS_REGISTER_SINKS {
            let cat = SinkCategory::from_sink_name(sink);
            if sink.contains("::") || sink.contains('.') {
                qualified_sink_names.insert(sink.to_string(), (cat, 100));
            } else {
                sink_names.insert(sink.to_string(), (cat, 100));
            }
        }

        Self {
            source_types: FxHashMap::default(),
            sink_names,
            qualified_sink_names,
            sanitizer_names: FxHashMap::default(),
        }
    }
}

impl CorpusSourceSinkRegistry {
    /// Check if a type annotation string is a known source type.
    pub fn is_source_type(&self, type_str: &str) -> bool {
        let clean = type_str.trim();
        self.source_types.contains_key(clean)
    }

    /// Check if a function name is a known sink (supports qualified names like "KV.put").
    pub fn is_sink(&self, qualified: &str) -> Option<SinkCategory> {
        if let Some((cat, _)) = self.qualified_sink_names.get(qualified) {
            return Some(*cat);
        }
        if let Some((cat, _)) = self.sink_names.get(qualified) {
            return Some(*cat);
        }
        let unqualified = qualified.rsplit('.').next().unwrap_or(qualified);
        if let Some((cat, count)) = self.sink_names.get(unqualified) {
            if *count >= 3 {
                return Some(*cat);
            }
        }
        None
    }

    /// Check if a full call expression string is a known sink.
    ///
    /// Checks in order:
    /// 1. Exact match on `sink_names` (bare function call like `exec`).
    /// 2. Exact match on `qualified_sink_names` (qualified like `collection.findOne`).
    /// 3. Suffix match: any entry in `sink_names` appears as the final `.`-delimited segment
    ///    of `expr`, guarded against safe built-in prefixes.
    pub fn is_sink_expr(&self, expr: &str) -> Option<SinkCategory> {
        // 1. Short-name exact match
        if let Some((cat, _)) = self.sink_names.get(expr) {
            return Some(*cat);
        }

        // 2. Qualified exact match
        if let Some((cat, _)) = self.qualified_sink_names.get(expr) {
            return Some(*cat);
        }

        // Safe built-in object prefixes — never a sink regardless of method name
        const SAFE_PREFIXES: &[&str] = &[
            "Object.", "Array.", "String.", "Number.", "Math.", "JSON.", "console.", "process.",
            "Promise.",
        ];
        for prefix in SAFE_PREFIXES {
            if expr.starts_with(prefix) {
                return None;
            }
        }

        // 3. Suffix match: last segment of a dotted call matches a known short sink
        if let Some(last_seg) = expr.rsplit('.').next() {
            // Strip any trailing call punctuation
            let clean = last_seg.trim_end_matches(|c: char| c == '(' || c == ')');
            if let Some((cat, _)) = self.sink_names.get(clean) {
                return Some(*cat);
            }
        }

        None
    }

    /// Check if a call expression is a known sanitizer.
    ///
    /// Returns `true` when the call is known to clean tainted input so taint
    /// propagation should stop at the result variable.
    pub fn is_sanitizer_call(&self, expr: &str) -> bool {
        // Learned from negative corpus
        if self.sanitizer_names.contains_key(expr) {
            return true;
        }
        // Built-in heuristics — stable regardless of corpus content
        const SANITIZER_FRAGMENTS: &[&str] = &[
            "escape",
            "sanitize",
            "encode",
            "validate",
            "strip",
            "clean",
            "purify",
            "filter",
            "dompurify",
            "xss",
            "he.",
        ];
        let lower = expr.to_lowercase();
        for frag in SANITIZER_FRAGMENTS {
            if lower.contains(frag) {
                return true;
            }
        }
        false
    }

    /// Get source type count (for diagnostics).
    pub fn source_type_count(&self) -> usize {
        self.source_types.len()
    }

    /// Get sink name count (for diagnostics).
    pub fn sink_name_count(&self) -> usize {
        self.sink_names.len()
    }

    /// Merge another registry into this one (accumulates counts).
    pub fn merge(&mut self, other: &CorpusSourceSinkRegistry) {
        for (k, v) in &other.source_types {
            *self.source_types.entry(k.clone()).or_insert(0) += v;
        }
        for (k, (cat, count)) in &other.sink_names {
            let entry = self.sink_names.entry(k.clone()).or_insert((*cat, 0));
            entry.1 += count;
        }
        for (k, (cat, count)) in &other.qualified_sink_names {
            let entry = self
                .qualified_sink_names
                .entry(k.clone())
                .or_insert((*cat, 0));
            entry.1 += count;
        }
        for (k, v) in &other.sanitizer_names {
            *self.sanitizer_names.entry(k.clone()).or_insert(0) += v;
        }
    }

    /// Prune entries below their specific threshold.
    pub fn prune(&mut self) {
        self.source_types.retain(|_, count| *count >= 2);

        self.sink_names
            .retain(|name, (_, count)| *count >= get_sink_tier(name).min_occurrences());

        self.qualified_sink_names
            .retain(|name, (_, count)| *count >= get_sink_tier(name).min_occurrences());

        self.sanitizer_names.retain(|_, count| *count >= 2);
    }
}

/// Build a registry from a set of positive source files.
///
/// Walks each file's AST to extract:
/// - Parameter type annotations → source types
/// - Call expression callee names → sink names
pub fn build_registry(positive_files: &[String]) -> CorpusSourceSinkRegistry {
    let mut registry = CorpusSourceSinkRegistry::default();

    for source in positive_files {
        let file_sources = extract_sources_from_source(source);
        let file_sinks = extract_sinks_from_source(source);

        // Count each type/sink once per file (not once per function)
        // to avoid over-counting multi-function positive files
        let mut seen_types = std::collections::HashSet::new();
        for ty in &file_sources {
            if seen_types.insert(ty.clone()) {
                *registry.source_types.entry(ty.clone()).or_insert(0) += 1;
            }
        }

        let mut seen_sinks = std::collections::HashSet::new();
        for sink in &file_sinks {
            let (short, qualified) = split_sink_name(sink);
            if seen_sinks.insert(short.clone()) {
                let cat = SinkCategory::from_sink_name(&short);
                let entry = registry.sink_names.entry(short).or_insert((cat, 0));
                entry.1 += 1;
            }
            if let Some(q) = qualified {
                let cat = SinkCategory::from_sink_name(&q);
                let entry = registry.qualified_sink_names.entry(q).or_insert((cat, 0));
                entry.1 += 1;
            }
        }
    }

    registry.prune();
    registry
}

/// Extract parameter name and type from a function parameter node.
///
/// Returns `(param_name, type_string)`. The type string includes the full
/// type annotation text (e.g., `"Request"`, `"Json<User>"`, `"String"`).
pub fn extract_param_info(param: tree_sitter::Node, source: &str) -> (String, String) {
    let mut name = String::new();
    let mut ty = String::new();

    let mut cursor = param.walk();
    for child in param.children(&mut cursor) {
        match child.kind() {
            "identifier" | "shorthand_field_identifier" | "field_identifier" if name.is_empty() => {
                name = source[child.start_byte()..child.end_byte()].to_string();
            }
            "type_annotation" | "type_identifier" | "scoped_type_identifier" | "generic_type"
                if ty.is_empty() =>
            {
                ty = source[child.start_byte()..child.end_byte()].to_string();
            }
            _ => {}
        }
    }

    // Fallback: regex on full text
    if name.is_empty() || ty.is_empty() {
        let text = &source[param.start_byte()..param.end_byte()];
        if let Some(caps) = regex::Regex::new(r"(\w+)\s*:\s*(.+)")
            .ok()
            .and_then(|re| re.captures(text))
        {
            if name.is_empty() {
                name = caps
                    .get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
            }
            if ty.is_empty() {
                ty = caps
                    .get(2)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
            }
        }
    }

    (name, ty)
}

/// Build a registry from a corpus directory by reading all positive files.
pub fn build_registry_from_dir(corpus_dir: &Path) -> CorpusSourceSinkRegistry {
    let mut positive_files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(corpus_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if name.contains("_positive.") {
                if let Ok(source) = std::fs::read_to_string(&path) {
                    positive_files.push(source);
                }
            }
        }
    }

    build_registry(&positive_files)
}

/// Extract source types from a source file by walking parameter type annotations.
fn extract_sources_from_source(source: &str) -> Vec<String> {
    let mut types = Vec::new();
    let mut parser = tree_sitter::Parser::new();

    let lang_name = if source.contains("fn ") {
        "rust"
    } else {
        "typescript"
    };

    let lang = crate::parser::ParserRegistry::get_language_by_name(lang_name).ok();
    let Some(lang) = lang else { return types };
    parser.set_language(&lang).ok();
    let Some(tree) = parser.parse(source, None) else {
        return types;
    };

    extract_param_types(tree.root_node(), source, &mut types);
    types
}

/// Extract sink function names from a source file by walking call expressions.
fn extract_sinks_from_source(source: &str) -> Vec<String> {
    let mut sinks = Vec::new();
    let mut parser = tree_sitter::Parser::new();

    let lang_name = if source.contains("fn ") {
        "rust"
    } else {
        "typescript"
    };

    let lang = crate::parser::ParserRegistry::get_language_by_name(lang_name).ok();
    let Some(lang) = lang else { return sinks };
    parser.set_language(&lang).ok();
    let Some(tree) = parser.parse(source, None) else {
        return sinks;
    };

    extract_call_names(tree.root_node(), source, &mut sinks);
    sinks
}

/// Recursively extract type annotations from function parameters.
fn extract_param_types(node: Node, source: &str, types: &mut Vec<String>) {
    let is_fn = matches!(
        node.kind(),
        "function_definition"     // TS
        | "function_declaration"  // TS
        | "arrow_function"        // TS
        | "method_definition"     // TS
        | "function_item"         // Rust
        | "function_signature_item" // Rust
    );

    if is_fn {
        if let Some(params) = node
            .child_by_field_name("parameters")
            .or_else(|| node.child_by_field_name("formal_parameters"))
        {
            let mut cursor = params.walk();
            for param in params.children(&mut cursor) {
                if matches!(param.kind(), "(" | ")" | "," | ";" | "self") {
                    continue;
                }
                extract_type_from_param(param, source, types);
            }
        }
    }

    // Recurse
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_param_types(cursor.node(), source, types);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Extract the type annotation from a single parameter node.
fn extract_type_from_param(param: Node, source: &str, types: &mut Vec<String>) {
    let mut cursor = param.walk();
    for child in param.children(&mut cursor) {
        match child.kind() {
            "type_annotation" | "type_identifier" | "scoped_type_identifier" | "generic_type" => {
                let ty = source[child.start_byte()..child.end_byte()].trim();
                if !ty.is_empty() {
                    let clean = ty.trim_start_matches(':').trim();
                    types.push(clean.to_string());
                }
            }
            _ => {}
        }
    }
}

/// Recursively extract function call names from the AST.
/// Pushes both the full qualified expression and the short method name so the
/// registry can do both exact and suffix matching.
fn extract_call_names(node: Node, source: &str, sinks: &mut Vec<String>) {
    if node.kind() == "call_expression" {
        if let Some(callee) = node
            .child_by_field_name("function")
            .or_else(|| node.child_by_field_name("callee"))
            .or_else(|| node.child(0))
        {
            // Push the full expression text as-is so qualified_sink_names gets it
            let full = source[callee.start_byte()..callee.end_byte()].to_string();
            if !full.is_empty() {
                sinks.push(full);
            }
        }
    }

    // Recurse
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_call_names(cursor.node(), source, sinks);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Split a raw callee expression string into `(short_name, qualified_name)`.
///
/// `short_name` is the final `.`-delimited segment (the method name).
/// `qualified_name` is the last two segments, used for context-aware matching
/// (e.g. `"db.collection(...).findOne"` → short=`"findOne"`, qualified=`"collection.findOne"`).
pub fn split_sink_name(raw: &str) -> (String, Option<String>) {
    // Strip call-argument suffixes like `(...)`
    let trimmed = if let Some(idx) = raw.find('(') {
        &raw[..idx]
    } else {
        raw
    };

    let parts: Vec<&str> = trimmed.split('.').collect();
    let short = parts
        .last()
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    let qualified = if parts.len() >= 2 {
        let last_two = &parts[parts.len() - 2..];
        // Skip if the penultimate segment contains parentheses (it's a chained call result)
        if !last_two[0].contains('(') {
            Some(format!("{}.{}", last_two[0].trim(), last_two[1].trim()))
        } else {
            None
        }
    } else {
        None
    };

    (short, qualified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_sources_typescript() {
        let source = "function handler(req: Request, body: Json<User>) { }";
        let types = extract_sources_from_source(source);
        assert!(
            types.contains(&"Request".to_string()),
            "should find Request type"
        );
        assert!(
            types.contains(&"Json<User>".to_string()),
            "should find Json<User> type"
        );
    }

    #[test]
    fn test_extract_sources_rust() {
        let source = "fn handler(req: Query<Params>) -> Response {\n    let x = req;\n}";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let mut types = Vec::new();
        extract_param_types(tree.root_node(), source, &mut types);
        assert!(
            !types.is_empty(),
            "should find at least one type, got: {types:?}"
        );
    }

    #[test]
    fn test_extract_sinks_typescript() {
        let source = "function handler() { exec(cmd); eval(code); }";
        let sinks = extract_sinks_from_source(source);
        assert!(sinks.contains(&"exec".to_string()), "should find exec sink");
        assert!(sinks.contains(&"eval".to_string()), "should find eval sink");
    }

    #[test]
    fn test_extract_sinks_member_expression() {
        let source = "function handler() { console.log(msg); document.write(html); }";
        let sinks = extract_sinks_from_source(source);
        assert!(
            sinks.iter().any(|s| s.contains("console.log")),
            "should find console.log"
        );
        assert!(
            sinks.iter().any(|s| s.contains("document.write")),
            "should find document.write"
        );
    }

    #[test]
    fn test_registry_pruning() {
        let mut registry = CorpusSourceSinkRegistry::default();
        registry.source_types.insert("Request".to_string(), 3);
        registry.source_types.insert("OneOff".to_string(), 1);
        registry
            .sink_names
            .insert("exec".to_string(), (SinkCategory::CodeExecution, 5));
        registry
            .sink_names
            .insert("rare_sink".to_string(), (SinkCategory::Unknown, 1));

        registry.prune();

        assert!(registry.is_source_type("Request"));
        assert!(!registry.is_source_type("OneOff"));
        assert!(registry.is_sink("exec").is_some());
        assert!(registry.is_sink("rare_sink").is_none());
    }

    #[test]
    fn test_build_registry() {
        let files = vec![
            "function handler(req: Request) { exec(req.query); }".to_string(),
            "function process(input: Request) { exec(input.data); }".to_string(),
        ];
        let registry = build_registry(&files);
        assert!(
            registry.source_types.contains_key("Request"),
            "Request should be a source type, got: {:?}",
            registry.source_types
        );
        assert!(
            registry.sink_names.contains_key("exec"),
            "exec should be a sink, got: {:?}",
            registry.sink_names
        );
    }
}
