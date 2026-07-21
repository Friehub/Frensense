// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::corpus::semantic::SemanticFilter;
use crate::fingerprint::{FunctionFingerprint, extract_fingerprints};

#[derive(Debug, Clone)]
pub struct CorpusPattern {
    pub id: String,
    pub positives: Vec<FunctionFingerprint>,
    pub negatives: Vec<FunctionFingerprint>,
    pub semantic_filter: Option<SemanticFilter>,
    pub observation: Option<String>,
    pub impact: Option<String>,
    pub improvement: Option<String>,
    pub expected_context: Option<crate::context::FileContext>,
}

pub fn load_corpus(corpus_dir: &Path) -> Result<Vec<CorpusPattern>, String> {
    type PatternEntry = (
        Vec<FunctionFingerprint>,
        Vec<FunctionFingerprint>,
        AdvisoryText,
        Vec<FunctionFeatures>, // positive features
        Vec<FunctionFeatures>, // negative features
    );
    let mut pairs: HashMap<String, PatternEntry> = HashMap::new();

    for entry in fs::read_dir(corpus_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        let is_positive = file_name.contains("_positive");
        // M1: Support _negative, _negative2, _negative3 ... for diverse negatives
        let is_negative = is_negative_file(file_name);

        if !is_positive && !is_negative {
            continue;
        }

        let source = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let lang_name = crate::parser::ext_to_language(ext);
        if lang_name == "unknown" {
            eprintln!(
                "corpus: skipping unsupported extension '{ext}' in '{}'",
                path.display()
            );
            continue;
        }

        let mut parser = tree_sitter::Parser::new();
        let lang = crate::parser::ParserRegistry::get_language_by_name(lang_name)
            .map_err(|e| e.to_string())?;
        parser.set_language(&lang).map_err(|e| e.to_string())?;
        let Some(tree) = parser.parse(&source, None) else {
            continue;
        };

        let mut fps = Vec::new();
        extract_fingerprints(tree.root_node(), &source, &path, &mut fps, 5);

        if fps.is_empty() {
            continue;
        }

        // Collect features from all function nodes for semantic learning
        let mut all_features = Vec::new();
        collect_all_function_features(tree.root_node(), &source, &mut all_features);

        let pattern_name = extract_pattern_name(file_name);
        let entry = pairs.entry(pattern_name).or_default();
        if is_positive {
            entry.0.extend(fps);
            entry.3.extend(all_features);
            // Extract [frensense] block from positive file — primary source of advisory text
            if entry.2.observation.is_none() {
                entry.2 = parse_frensense_block(&source);
            }
            // M4: Auto-infer expected_context from the positive file path+content — no TOML needed
            if entry.2.expected_context.is_none() {
                entry.2.expected_context =
                    Some(crate::context::FileContext::extract(&path, &source));
            }
        } else {
            entry.1.extend(fps);
            entry.4.extend(all_features);
        }
    }

    let mut patterns = Vec::new();
    let semantic_filters = load_semantic_filters();
    for (name, (pos, neg, comment_advisory, pos_features, neg_features)) in pairs {
        if pos.is_empty() && neg.is_empty() {
            continue;
        }
        if pos.is_empty() {
            eprintln!("Corpus warning: pattern '{name}' has negative but no positive example");
            continue;
        }
        if neg.is_empty() {
            eprintln!("Corpus warning: pattern '{name}' has positive but no negative example");
            continue;
        }

        // Priority: comment block > sidecar TOML (optional override) > synthesized
        // The sidecar TOML is NEVER required — it is only an escape hatch for edge cases.
        let toml_advisory = load_sidecar_toml(corpus_dir, &name);

        // Learn semantic constraints from positive/negative examples
        let learned = if !pos_features.is_empty() && !neg_features.is_empty() {
            // M2: Pass taint source awareness into constraint learning
            learn_from_features(&pos_features, &neg_features)
        } else {
            crate::corpus::semantic::LearnedConstraints::default()
        };

        // M3: Synthesize advisory text from learned constraints when no comment block is present
        let synthesized =
            synthesize_advisory(&name, &learned.required_calls, &learned.forbidden_calls);

        let observation = comment_advisory
            .observation
            .or(toml_advisory.observation)
            .or(synthesized.observation);
        let impact = comment_advisory
            .impact
            .or(toml_advisory.impact)
            .or(synthesized.impact);
        let improvement = comment_advisory
            .improvement
            .or(toml_advisory.improvement)
            .or(synthesized.improvement);

        // M4: Auto-context is already in comment_advisory.expected_context (set during file scan).
        // Sidecar TOML is a manual override; auto-inferred value is the fallback.
        let expected_context = toml_advisory
            .expected_context
            .or(comment_advisory.expected_context);

        // Merge: TOML manual filter takes precedence over learned constraints
        let filter = if let Some(manual) = semantic_filters.get(&name) {
            Some(manual.clone())
        } else if !learned.is_empty() {
            Some(learned.to_filter())
        } else {
            None
        };

        patterns.push(CorpusPattern {
            id: name.clone(),
            positives: pos,
            negatives: neg,
            semantic_filter: filter,
            observation,
            impact,
            improvement,
            expected_context,
        });
    }

    Ok(patterns)
}

#[derive(Debug, Clone, Default)]
struct AdvisoryText {
    observation: Option<String>,
    impact: Option<String>,
    improvement: Option<String>,
    expected_context: Option<crate::context::FileContext>,
}

/// M3: Synthesize advisory text from what the AST diff already tells us.
/// Used as a fallback when no `[frensense]` comment block is present in the positive file.
fn synthesize_advisory(
    pattern_id: &str,
    required_calls: &[String],
    forbidden_calls: &[String],
) -> AdvisoryText {
    // Convert pattern_id like "ts_jwt_bypass" to a readable label
    let label = pattern_id.replace('_', " ");
    let observation = if required_calls.is_empty() {
        format!("Pattern '{label}' matches a known vulnerability shape.")
    } else {
        format!(
            "Function calls {}. This matches a known vulnerability ({label}).",
            required_calls.join(", ")
        )
    };
    let improvement = if forbidden_calls.is_empty() {
        "Review the function against the corpus positive example.".to_string()
    } else {
        format!(
            "Replace {} with {} and validate all inputs.",
            required_calls.join(" / "),
            forbidden_calls.join(" / ")
        )
    };
    AdvisoryText {
        observation: Some(observation),
        impact: None,
        improvement: Some(improvement),
        expected_context: None,
    }
}

/// Parse a `/// [frensense]` / `// [frensense]` / `# [frensense]` block from source.
///
/// Format:
/// ```text
/// [frensense]
/// observation: what the bug looks like
/// impact: what goes wrong
/// improvement: how to fix it
/// ```
///
/// Block ends at the first blank comment line or a non-comment line.
fn parse_frensense_block(source: &str) -> AdvisoryText {
    let mut result = AdvisoryText::default();
    let mut in_block = false;

    for line in source.lines() {
        let trimmed = line.trim();

        // Detect comment prefix
        let content = if let Some(c) = trimmed.strip_prefix("///") {
            Some(c.trim())
        } else if let Some(c) = trimmed.strip_prefix("//!") {
            Some(c.trim())
        } else if let Some(c) = trimmed.strip_prefix("//") {
            Some(c.trim())
        } else {
            trimmed.strip_prefix("#").map(str::trim)
        };

        let Some(text) = content else {
            // Non-comment line — block is over
            break;
        };

        if !in_block {
            if text == "[frensense]" {
                in_block = true;
            }
            continue;
        }

        // Empty comment line ends the block
        if text.is_empty() {
            break;
        }

        if let Some((key, value)) = text.split_once(':') {
            let key = key.trim().to_lowercase();
            let value = value.trim().to_string();
            if !value.is_empty() {
                match key.as_str() {
                    "observation" => result.observation = Some(value),
                    "impact" => result.impact = Some(value),
                    "improvement" => result.improvement = Some(value),
                    _ => {}
                }
            }
        }
    }

    result
}

fn load_sidecar_toml(corpus_dir: &std::path::Path, pattern_name: &str) -> AdvisoryText {
    let toml_path = corpus_dir.join(format!("{pattern_name}.toml"));
    let Ok(content) = std::fs::read_to_string(&toml_path) else {
        return AdvisoryText::default();
    };

    let Ok(doc) = content.parse::<toml::Table>() else {
        return AdvisoryText::default();
    };

    let expected_context = doc
        .get("expected_context")
        .and_then(|t| t.as_table())
        .map(|t| {
            let env_str = t
                .get("environment")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            let sens_str = t
                .get("sensitivity")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");

            let env = match env_str {
                "Test" => crate::context::Environment::Test,
                "Mock" => crate::context::Environment::Mock,
                "RouteHandler" => crate::context::Environment::RouteHandler,
                "Utility" => crate::context::Environment::Utility,
                "Config" => crate::context::Environment::Config,
                _ => crate::context::Environment::Unknown,
            };

            let sens = match sens_str {
                "Low" => crate::context::DataSensitivity::Low,
                "Medium" => crate::context::DataSensitivity::Medium,
                "High" => crate::context::DataSensitivity::High,
                _ => crate::context::DataSensitivity::Unknown,
            };

            let frameworks = t
                .get("frameworks")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            crate::context::FileContext {
                environment: env,
                sensitivity: sens,
                frameworks,
            }
        });

    AdvisoryText {
        observation: doc
            .get("observation")
            .and_then(|v| v.as_str())
            .map(String::from),
        impact: doc.get("impact").and_then(|v| v.as_str()).map(String::from),
        improvement: doc
            .get("improvement")
            .and_then(|v| v.as_str())
            .map(String::from),
        expected_context,
    }
}

/// Load semantic filters from the TOML file.
pub fn load_semantic_filters() -> std::collections::HashMap<String, SemanticFilter> {
    let mut filters = std::collections::HashMap::new();
    filters.insert(
        "rust_csa_auth_no_rejection".to_string(),
        SemanticFilter {
            contains_call_to: vec![
                "decode".to_string(),
                "verify".to_string(),
                "token".to_string(),
                "jwt".to_string(),
                "exp".to_string(),
            ],
            function_name_regex: Some("^authenticate".to_string()),
            ..Default::default()
        },
    );
    filters.insert(
        "rust_csa_find_never_empty".to_string(),
        SemanticFilter {
            must_not_contain_node_type: vec!["return_statement".to_string()],
            function_name_regex: Some("^find".to_string()),
            ..Default::default()
        },
    );
    filters.insert(
        "rust_csa_validate_unconditional".to_string(),
        SemanticFilter {
            function_name_regex: Some("^validate".to_string()),
            ..Default::default()
        },
    );
    filters.insert(
        "rust_csa_sanitize_passthrough".to_string(),
        SemanticFilter {
            must_not_contain_call_to: vec![
                ".replace".to_string(),
                ".encode".to_string(),
                "escape".to_string(),
            ],
            function_name_regex: Some("^sanitize".to_string()),
            ..Default::default()
        },
    );
    filters.insert(
        "rust_cvefixes_no_resource_limit_cve-2026-27572_headers".to_string(),
        SemanticFilter {
            contains_call_to: vec![
                "downcast".to_string(),
                "push_child".to_string(),
                "get".to_string(),
                "table".to_string(),
            ],
            ..Default::default()
        },
    );
    filters.insert(
        "rust_cvefixes_no_resource_limit_cve-2026-27572_build".to_string(),
        SemanticFilter {
            contains_call_to: vec!["build".to_string(), "new".to_string(), "push".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "rust_cvefixes_no_resource_limit_cve-2026-27572_configure_wasip2".to_string(),
        SemanticFilter {
            contains_call_to: vec!["configure".to_string(), "wasi".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "rust_cvefixes_no_resource_limit_cve-2026-27572_default".to_string(),
        SemanticFilter {
            contains_call_to: vec!["default".to_string(), "new".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_llm_promise_catch".to_string(),
        SemanticFilter {
            contains_call_to: vec![".then".to_string()],
            must_not_contain_call_to: vec![".catch".to_string(), ".finally".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_csa_sanitize_passthrough".to_string(),
        SemanticFilter {
            must_not_contain_call_to: vec![
                ".replace".to_string(),
                ".encode".to_string(),
                "encodeURIComponent".to_string(),
                "URL".to_string(),
            ],
            function_name_regex: Some("^sanitize".to_string()),
            ..Default::default()
        },
    );
    filters.insert(
        "ts_llm_console_log".to_string(),
        SemanticFilter {
            contains_call_to: vec!["logger.info".to_string(), "console.log".to_string()],
            must_not_contain_call_to: vec!["structuredLogger".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_llm_any_parameter".to_string(),
        SemanticFilter {
            contains_node_type: vec![
                "required_parameter".to_string(),
                "optional_parameter".to_string(),
                "type_annotation".to_string(),
            ],
            ..Default::default()
        },
    );
    filters.insert(
        "rust_clone_in_loop".to_string(),
        SemanticFilter {
            contains_call_to: vec![".clone".to_string()],
            contains_node_type: vec![
                "loop_expression".to_string(),
                "for_expression".to_string(),
                "while_expression".to_string(),
            ],
            ..Default::default()
        },
    );
    filters.insert(
        "rust_async_blocking_io".to_string(),
        SemanticFilter {
            contains_call_to: vec![
                "std::fs".to_string(),
                "std::io::read".to_string(),
                "std::io::write".to_string(),
                "File::open".to_string(),
                "std::thread::sleep".to_string(),
            ],
            contains_node_type: vec!["async_block".to_string(), "async".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "rust_llm_await_in_sync".to_string(),
        SemanticFilter {
            contains_node_type: vec!["await_expression".to_string()],
            must_not_contain_node_type: vec!["async".to_string(), "async_block".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "rust_connection_leak".to_string(),
        SemanticFilter {
            contains_call_to: vec![
                "connect".to_string(),
                "Connection::new".to_string(),
                "pool.get".to_string(),
            ],
            must_not_contain_call_to: vec!["drop".to_string(), "close".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "rust_network_in_txn".to_string(),
        SemanticFilter {
            contains_call_to: vec!["begin_transaction".to_string(), "fetch".to_string()],
            must_not_contain_call_to: vec!["update_db".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "rust_mutate_after_response".to_string(),
        SemanticFilter {
            contains_call_to: vec![
                "send".to_string(),
                "respond".to_string(),
                "into_response".to_string(),
            ],
            ..Default::default()
        },
    );
    filters.insert(
        "rust_transmute".to_string(),
        SemanticFilter {
            contains_call_to: vec!["transmute".to_string(), "transmute_copy".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "rust_llm_clone_literal".to_string(),
        SemanticFilter {
            contains_call_to: vec!["clone".to_string()],
            contains_node_type: vec!["integer_literal".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_hardcoded_secret".to_string(),
        SemanticFilter {
            contains_node_type: vec!["string".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_prototype_pollution".to_string(),
        SemanticFilter {
            contains_call_to: vec![
                "__proto__".to_string(),
                "constructor".to_string(),
                "prototype".to_string(),
            ],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_sql_injection".to_string(),
        SemanticFilter {
            contains_call_to: vec![
                "query".to_string(),
                "execute".to_string(),
                "raw".to_string(),
            ],
            contains_node_type: vec![
                "template_string".to_string(),
                "binary_expression".to_string(),
            ],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_command_injection".to_string(),
        SemanticFilter {
            contains_call_to: vec![
                "exec".to_string(),
                "spawn".to_string(),
                "system".to_string(),
                "execSync".to_string(),
                "spawnSync".to_string(),
            ],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_ssrf".to_string(),
        SemanticFilter {
            contains_call_to: vec![
                "fetch".to_string(),
                "axios".to_string(),
                "request".to_string(),
                "http.get".to_string(),
                "https.get".to_string(),
            ],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_path_traversal".to_string(),
        SemanticFilter {
            contains_call_to: vec![
                "readFile".to_string(),
                "writeFile".to_string(),
                "unlink".to_string(),
                "stat".to_string(),
                "access".to_string(),
                "path.join".to_string(),
            ],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_open_redirect".to_string(),
        SemanticFilter {
            contains_call_to: vec![
                "redirect".to_string(),
                "location.href".to_string(),
                "window.location".to_string(),
            ],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_cookie_security".to_string(),
        SemanticFilter {
            contains_call_to: vec![
                "cookie".to_string(),
                "setCookie".to_string(),
                "document.cookie".to_string(),
            ],
            must_not_contain_node_type: vec!["object".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_csa_validate_unconditional".to_string(),
        SemanticFilter {
            must_not_contain_node_type: vec!["false".to_string()],
            function_name_regex: Some("^validate".to_string()),
            ..Default::default()
        },
    );
    filters.insert(
        "typescript_cvefixes_code_injection_cve-2026-27702_runView".to_string(),
        SemanticFilter {
            contains_call_to: vec!["eval".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "typescript_cvefixes_code_injection_cve-2026-27702_isEmptyExpression".to_string(),
        SemanticFilter {
            contains_call_to: vec!["eval".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "typescript_cvefixes_code_injection_cve-2026-27702_migrateToDesignView".to_string(),
        SemanticFilter {
            contains_call_to: vec!["eval".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "typescript_cvefixes_code_injection_cve-2026-27702_migrateToInMemoryView".to_string(),
        SemanticFilter {
            contains_call_to: vec!["eval".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "typescript_cvefixes_code_injection_cve-2026-27702_parseFilterExpression".to_string(),
        SemanticFilter {
            contains_call_to: vec!["eval".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "typescript_cvefixes_xss_cve-2026-27148_storybookDevServer".to_string(),
        SemanticFilter {
            contains_call_to: vec!["app.use".to_string(), "server".to_string()],
            contains_node_type: vec!["await_expression".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "typescript_cvefixes_xss_cve-2026-27148_getAccessControlMiddleware".to_string(),
        SemanticFilter {
            contains_call_to: vec!["middleware".to_string(), "header".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "typescript_cvefixes_xss_cve-2026-27148_getServerChannel".to_string(),
        SemanticFilter {
            contains_call_to: vec!["channel".to_string(), "server".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "typescript_cvefixes_xss_cve-2026-27148_handleConnected".to_string(),
        SemanticFilter {
            contains_call_to: vec!["handle".to_string(), "connected".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "typescript_cvefixes_xss_cve-2026-27148_main".to_string(),
        SemanticFilter {
            contains_call_to: vec!["app".to_string(), "server".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_deserialization".to_string(),
        SemanticFilter {
            contains_call_to: vec!["JSON.parse".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_unauthenticated_db_write".to_string(),
        SemanticFilter {
            must_not_match_function_name: vec![
                "register".to_string(),
                "signup".to_string(),
                "createAccount".to_string(),
                "resetPassword".to_string(),
                "forgotPassword".to_string(),
                "verify".to_string(),
                "confirm".to_string(),
            ],
            must_not_match_file_path_pattern: vec![
                "*.test.ts".to_string(),
                "*.spec.ts".to_string(),
                "__tests__/".to_string(),
                "fixtures/".to_string(),
            ],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_missing_payment_gate".to_string(),
        SemanticFilter {
            must_not_match_function_name: vec![
                "healthCheck".to_string(),
                "handleWebhook".to_string(),
                "getStatus".to_string(),
                "handleOptions".to_string(),
                "preflight".to_string(),
            ],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_race_condition_read_check_write".to_string(),
        SemanticFilter {
            must_not_match_file_path_pattern: vec![
                "*.test.ts".to_string(),
                "*.spec.ts".to_string(),
                "test-utils/".to_string(),
            ],
            ..Default::default()
        },
    );
    // === Distintive-sink semantic filters ===
    // These patterns require the candidate function to call a specific API.
    // If the candidate calls res.redirect() but not exec(), CMDI shouldn't match.

    let sink_patterns: Vec<(&str, Vec<&str>)> = vec![
        ("ts_cmdi_exec_direct", vec!["exec", "spawn", "execFile", "fork"]),
        ("ts_cmdi_exec_direct_m2", vec!["exec", "spawn", "execFile"]),
        ("ts_cmdi_exec_direct_m4", vec!["exec", "spawn", "execFile"]),
        ("ts_cmdi_exec_direct_m10", vec!["exec", "spawn", "execFile"]),
        ("ts_cmdi_template_literal", vec!["exec", "spawn", "execFile"]),
        ("ts_cmdi_template_literal_m4", vec!["exec", "spawn", "execFile"]),
        ("ts_cmdi_template_literal_m10", vec!["exec", "spawn", "execFile"]),
        ("ts_cmdi_spawn_args", vec!["spawn", "exec", "execFile"]),
        ("ts_cmdi_spawn_args_m4", vec!["spawn", "exec", "execFile"]),
        ("ts_cmdi_shell_true", vec!["exec", "spawn", "execFile"]),
        ("ts_cmdi_env_injection", vec!["exec", "spawn"]),
        ("ts_ssrf_fetch_direct", vec!["fetch", "request", "get"]),
        ("ts_ssrf_fetch_direct_m4", vec!["fetch", "request"]),
        ("ts_ssrf_fetch_direct_m10", vec!["fetch", "request"]),
        ("ts_ssrf_fetch_constructed", vec!["fetch", "request"]),
        ("ts_ssrf_fetch_constructed_m4", vec!["fetch", "request"]),
        ("ts_path_traversal_readfile", vec!["readFile", "writeFile", "createReadStream"]),
        ("ts_path_traversal_readfile_m10", vec!["readFile", "writeFile"]),
        ("ts_path_traversal_join_no_check", vec!["readFile", "writeFile", "join"]),
        ("ts_path_traversal_join_no_check_m4", vec!["readFile", "writeFile"]),
        ("ts_eval_direct", vec!["eval"]),
        ("ts_eval_direct_m4", vec!["eval"]),
        ("ts_nosqli_mongo_where", vec!["$where", "find", "findOne"]),
        ("ts_nosqli_mongo_where_m4", vec!["$where", "find"]),
        ("ts_sqli_prisma_query_raw_unsafe", vec!["$queryRawUnsafe", "queryRaw", "executeRaw"]),
        ("ts_sqli_prisma_query_raw_unsafe_m4", vec!["$queryRawUnsafe", "queryRaw"]),
        ("ts_sqli_knex_raw", vec!["raw", "knex"]),
        ("ts_sqli_knex_raw_m4", vec!["raw", "knex"]),
        ("ts_sqli_concat_direct", vec!["query", "concat", "execute", "find"]),
        ("ts_sqli_concat_direct_m4", vec!["query", "execute"]),
        ("ts_idor_update_no_ownership", vec!["update", "findAndModify", "exec"]),
        ("ts_idor_update_no_ownership_m4", vec!["update", "findAndModify"]),
        ("ts_idor_child_resource", vec!["find", "findOne", "query"]),
        ("ts_idor_child_resource_m4", vec!["find", "findOne"]),
        ("ts_prototype_pollution_merge", vec!["merge", "assign", "extend"]),
        ("ts_prototype_pollution_merge_m4", vec!["merge", "assign"]),
        ("ts_tanstack_mutation_on_success_stale_closure", vec!["useMutation", "mutate", "queryClient"]),
        ("ts_oauth_state_reuse", vec!["state", "oauth", "redirect_uri", "authorize"]),
        ("ts_oauth_pkce_missing", vec!["state", "oauth", "code_challenge"]),
        ("ts_oauth_missing_state", vec!["state", "oauth", "authorize"]),
        ("ts_token_in_url_fragment", vec!["fragment", "hash", "oauth"]),
        ("tsx_xss_href_javascript", vec!["href", "window.open", "location"]),
        ("tsx_xss_href_javascript_m8", vec!["href", "window.open"]),
        ("tsx_xss_href_javascript_m6", vec!["href", "window.open"]),
        ("tsx_xss_href_javascript_m5", vec!["href", "window.open"]),
        ("tsx_xss_href_javascript_m2", vec!["href", "window.open"]),
        ("tsx_xss_href_javascript_m3", vec!["href", "window.open"]),
        ("tsx_image_src_unvalidated", vec!["src", "image", "img"]),
        ("ts_sqli_function_built", vec!["sql", "query", "concat", "select", "from"]),
    ];

    for (pattern_id, sinks) in sink_patterns {
        filters.insert(
            pattern_id.to_string(),
            SemanticFilter {
                contains_call_to: sinks.iter().map(|s| s.to_string()).collect(),
                ..Default::default()
            },
        );
    }

    // Patterns with no distinctive sink calls — use function name or file path guards
    filters.insert(
        "ts_llm_system_prompt_in_client".to_string(),
        SemanticFilter {
            function_name_regex: Some("prompt".to_string()),
            ..Default::default()
        },
    );
    filters.insert(
        "ts_perm_cache_stale_elevation".to_string(),
        SemanticFilter {
            contains_call_to: vec!["redis".to_string(), "cache".to_string(), "memcached".to_string()],
            ..Default::default()
        },
    );
    filters.insert(
        "ts_cache_unkeyed_header".to_string(),
        SemanticFilter {
            must_not_match_file_path_pattern: vec!["routes/".to_string()],
            ..Default::default()
        },
    );

    filters
}

/// Returns true if a file name represents any negative variant:
/// `_negative.ts`, `_negative2.ts`, `_negative3.ts`, etc.
fn is_negative_file(file_name: &str) -> bool {
    // Strip the extension first, then check suffix
    let stem = file_name.rsplitn(2, '.').last().unwrap_or(file_name);
    if stem.ends_with("_negative") {
        return true;
    }
    // Match _negative2, _negative3, ... _negative9
    if let Some(prefix) = stem.strip_suffix(|c: char| c.is_ascii_digit()) {
        if prefix.ends_with("_negative") {
            return true;
        }
    }
    false
}

fn extract_pattern_name(file_name: &str) -> String {
    let without_ext = file_name.rsplitn(2, '.').last().unwrap_or(file_name);

    // Positive files: just strip _positive suffix (single occurrence)
    if let Some(stripped) = without_ext.strip_suffix("_positive") {
        return stripped.to_string();
    }

    // Negative files: strip _negative, _negative2 ... _negative9 (single occurrence)
    if let Some(stripped) = without_ext.strip_suffix("_negative") {
        return stripped.to_string();
    }
    if let Some(digits) = without_ext.strip_suffix(|c: char| c.is_ascii_digit()) {
        if let Some(stripped) = digits.strip_suffix("_negative") {
            return stripped.to_string();
        }
    }

    without_ext.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_pattern_name() {
        assert_eq!(
            extract_pattern_name("rust_clone_in_loop_positive.rs"),
            "rust_clone_in_loop"
        );
        assert_eq!(
            extract_pattern_name("ts_command_injection_negative.ts"),
            "ts_command_injection"
        );
    }

    #[test]
    fn test_empty_directory_returns_empty() {
        let dir = tempfile::tempdir().unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_nonexistent_directory_returns_error() {
        let result = load_corpus(std::path::Path::new("/nonexistent/path/xyz"));
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_file_skipped_silently() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("test_positive.ts"), "").unwrap();
        std::fs::write(dir.path().join("test_negative.ts"), "").unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(patterns.is_empty(), "Empty files should be skipped");
    }

    #[test]
    fn test_no_function_body_skipped_silently() {
        let dir = tempfile::tempdir().unwrap();
        // Type declaration only — no function body
        std::fs::write(
            dir.path().join("test_positive.ts"),
            "interface Config { host: string; }",
        )
        .unwrap();
        std::fs::write(dir.path().join("test_negative.ts"), "type Foo = string;").unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(
            patterns.is_empty(),
            "Files without functions should be skipped"
        );
    }

    #[test]
    fn test_bad_syntax_skipped_silently() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("bad_positive.ts"), "fn {{{ broken").unwrap();
        std::fs::write(dir.path().join("bad_negative.ts"), "fn {{{ broken").unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(
            patterns.is_empty(),
            "Files with bad syntax should be skipped"
        );
    }

    #[test]
    fn test_unsupported_extension_skipped() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("test_positive.xyz"), "def foo(): pass").unwrap();
        std::fs::write(dir.path().join("test_negative.xyz"), "def bar(): pass").unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(
            patterns.is_empty(),
            "Unsupported extensions should be skipped"
        );
    }

    #[test]
    fn test_only_positive_no_warning_no_crash() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("solo_positive.ts"),
            "function foo() { return 1; }",
        )
        .unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(
            patterns.is_empty(),
            "Positive-only should not produce a pattern"
        );
    }

    #[test]
    fn test_only_negative_no_warning_no_crash() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("solo_negative.ts"),
            "function bar() { return 2; }",
        )
        .unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(
            patterns.is_empty(),
            "Negative-only should not produce a pattern"
        );
    }

    #[test]
    fn test_non_function_files_ignored() {
        let dir = tempfile::tempdir().unwrap();
        // Files without _positive/_negative in name should be ignored
        std::fs::write(dir.path().join("readme.md"), "hello").unwrap();
        std::fs::write(dir.path().join("config.json"), "{}").unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_parse_frensense_block_rust() {
        let source = r"/// [frensense]
/// observation: Function always returns true regardless of input.
/// impact: Malicious input passes validation unchecked.
/// improvement: Branch on input and return false for invalid values.
fn validate(input: &str) -> bool {
    true
}";
        let advisory = parse_frensense_block(source);
        assert_eq!(
            advisory.observation.as_deref(),
            Some("Function always returns true regardless of input.")
        );
        assert_eq!(
            advisory.impact.as_deref(),
            Some("Malicious input passes validation unchecked.")
        );
        assert_eq!(
            advisory.improvement.as_deref(),
            Some("Branch on input and return false for invalid values.")
        );
    }

    #[test]
    fn test_parse_frensense_block_typescript() {
        let source = "// [frensense]\n// observation: sanitize returns input unchanged.\n// impact: XSS payload passes through.\n// improvement: HTML-escape entities.\n";
        let advisory = parse_frensense_block(source);
        assert_eq!(
            advisory.observation.as_deref(),
            Some("sanitize returns input unchanged.")
        );
        assert_eq!(
            advisory.impact.as_deref(),
            Some("XSS payload passes through.")
        );
        assert_eq!(
            advisory.improvement.as_deref(),
            Some("HTML-escape entities.")
        );
    }

    #[test]
    fn test_parse_frensense_block_python() {
        let source = "# [frensense]\n# observation: No rejection on invalid token.\n# impact: Auth bypass.\n# improvement: Return None on failure.\n";
        let advisory = parse_frensense_block(source);
        assert_eq!(
            advisory.observation.as_deref(),
            Some("No rejection on invalid token.")
        );
        assert_eq!(advisory.impact.as_deref(), Some("Auth bypass."));
        assert_eq!(
            advisory.improvement.as_deref(),
            Some("Return None on failure.")
        );
    }

    #[test]
    fn test_parse_frensense_block_blank_line_ends() {
        let source = "/// [frensense]\n/// observation: Bug here.\n\n/// impact: Overwritten.\n";
        let advisory = parse_frensense_block(source);
        assert_eq!(advisory.observation.as_deref(), Some("Bug here."));
        assert_eq!(advisory.impact, None, "blank line should end the block");
    }

    #[test]
    fn test_parse_frensense_block_no_block() {
        let source = "fn foo() { return 1; }";
        let advisory = parse_frensense_block(source);
        assert!(advisory.observation.is_none());
        assert!(advisory.impact.is_none());
        assert!(advisory.improvement.is_none());
    }

    #[test]
    fn test_parse_frensense_block_partial() {
        let source = "/// [frensense]\n/// observation: Only observation provided.\n";
        let advisory = parse_frensense_block(source);
        assert_eq!(
            advisory.observation.as_deref(),
            Some("Only observation provided.")
        );
        assert!(advisory.impact.is_none());
        assert!(advisory.improvement.is_none());
    }

    #[test]
    fn test_valid_pair_loads_correctly() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("rust_foo_positive.rs"),
            "fn foo() -> i32 { 1 }",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("rust_foo_negative.rs"),
            "fn foo() -> i32 { 2 }",
        )
        .unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].id, "rust_foo");
        assert!(!patterns[0].positives.is_empty());
        assert!(!patterns[0].negatives.is_empty());
    }

    #[test]
    fn test_multi_function_positive_loads_all() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("rust_multi_positive.rs"),
            "fn a() { panic!(\"x\"); }\nfn b() { panic!(\"y\"); }\nfn c() { panic!(\"z\"); }\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("rust_multi_negative.rs"),
            "fn a() -> Result<(), String> { Ok(()) }\nfn b() -> Result<(), String> { Ok(()) }\n",
        )
        .unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(
            patterns[0].positives.len(),
            3,
            "should extract all 3 functions from positive"
        );
        assert_eq!(
            patterns[0].negatives.len(),
            2,
            "should extract all 2 functions from negative"
        );
    }
}

/// Known taint source access patterns — user-controlled input entry points.
/// If a positive example contains these and the negative does not, the pattern
/// requires taint access to match (auto-promotes to SemanticFilter.contains_call_to).
pub const TAINT_SOURCE_PATTERNS: &[&str] = &[
    "req.query",
    "req.body",
    "req.params",
    "req.headers",
    "req.cookies",
    "ctx.request",
    "ctx.query",
    "ctx.params",
    "ctx.body",
    "event.body",
    "request.body",
    "request.query",
    "process.argv",
    "c.req",
];

/// Collected features from a function node for constraint learning.
#[derive(Debug, Clone, Default)]
struct FunctionFeatures {
    calls: Vec<String>,
    node_types: Vec<String>,
    /// M2: Set if the function reads from a recognized taint source (user-controlled input)
    taint_sources: Vec<String>,
}

/// Collect features from a function node.
fn collect_function_features(node: tree_sitter::Node<'_>, source: &str) -> FunctionFeatures {
    let mut features = FunctionFeatures::default();

    // Collect call targets
    let mut cursor = node.walk();
    loop {
        let n = cursor.node();
        if n.kind() == "call_expression" {
            if let Some(callee) = n
                .child_by_field_name("function")
                .or_else(|| n.child_by_field_name("callee"))
            {
                let target = source[callee.start_byte()..callee.end_byte()].to_string();
                features.calls.push(target);
            }
        }

        // Collect node types (only meaningful ones)
        let kind = n.kind();
        if !kind.is_empty() && !kind.starts_with("comment") {
            features.node_types.push(kind.to_string());
        }

        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                break;
            }
        }
        if !cursor.goto_first_child() {
            break;
        }
    }

    features.calls.sort();
    features.calls.dedup();
    features.node_types.sort();
    features.node_types.dedup();

    // M2: Detect taint sources by scanning the raw source text of this function's span
    let func_src = &source[node.start_byte()..node.end_byte().min(source.len())];
    for &pattern in TAINT_SOURCE_PATTERNS {
        if func_src.contains(pattern) {
            features.taint_sources.push(pattern.to_string());
        }
    }
    features.taint_sources.sort();
    features.taint_sources.dedup();

    features
}

/// Collect features from all function nodes in an AST.
fn collect_all_function_features(
    node: tree_sitter::Node<'_>,
    source: &str,
    out: &mut Vec<FunctionFeatures>,
) {
    let kind = node.kind();
    if kind == "function_item"
        || kind == "function_declaration"
        || kind == "method_definition"
        || kind == "arrow_function"
        || kind == "function"
        || kind == "generator_function"
        || kind == "function_signature"
        || kind == "method_declaration"
    {
        out.push(collect_function_features(node, source));
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            collect_all_function_features(child, source, out);
        }
    }
}

/// Learn semantic constraints from pre-collected features.
fn learn_from_features(
    pos_features: &[FunctionFeatures],
    neg_features: &[FunctionFeatures],
) -> crate::corpus::semantic::LearnedConstraints {
    if pos_features.is_empty() || neg_features.is_empty() {
        return crate::corpus::semantic::LearnedConstraints::default();
    }

    // Collect all call targets from positives and negatives
    let pos_calls: Vec<&str> = pos_features
        .iter()
        .flat_map(|f| f.calls.iter().map(std::string::String::as_str))
        .collect();
    let neg_calls: Vec<&str> = neg_features
        .iter()
        .flat_map(|f| f.calls.iter().map(std::string::String::as_str))
        .collect();

    // Find calls in ALL positives but NOT in any negative
    let mut required_calls: Vec<String> = pos_features[0]
        .calls
        .iter()
        .filter(|call| {
            pos_features.iter().all(|f| f.calls.contains(*call))
                && !neg_calls.contains(&call.as_str())
        })
        .cloned()
        .collect();

    // M2: Auto-promote taint sources to required_calls when positives have taint
    // and negatives do not — eliminates FP on non-user-controlled code paths.
    let pos_has_taint = pos_features.iter().any(|f| !f.taint_sources.is_empty());
    let neg_has_taint = neg_features.iter().any(|f| !f.taint_sources.is_empty());
    if pos_has_taint && !neg_has_taint {
        // Collect taint sources present in any positive but absent from all negatives
        let neg_taint: std::collections::HashSet<&str> = neg_features
            .iter()
            .flat_map(|f| f.taint_sources.iter().map(std::string::String::as_str))
            .collect();
        for f in pos_features {
            for src in &f.taint_sources {
                if !neg_taint.contains(src.as_str()) && !required_calls.contains(src) {
                    required_calls.push(src.clone());
                }
            }
        }
    }

    // Find calls in ALL negatives but NOT in any positive
    let forbidden_calls: Vec<String> = neg_features[0]
        .calls
        .iter()
        .filter(|call| {
            neg_features.iter().all(|f| f.calls.contains(*call))
                && !pos_calls.contains(&call.as_str())
        })
        .cloned()
        .collect();

    // Same for node types
    let pos_nts: Vec<&str> = pos_features
        .iter()
        .flat_map(|f| f.node_types.iter().map(std::string::String::as_str))
        .collect();
    let neg_nts: Vec<&str> = neg_features
        .iter()
        .flat_map(|f| f.node_types.iter().map(std::string::String::as_str))
        .collect();

    // Filter out noise node types
    let noise: std::collections::HashSet<&str> = [
        "program",
        "statement_block",
        "expression_statement",
        "return_statement",
        "if_statement",
        "variable_declaration",
        "identifier",
        "call_expression",
        "member_expression",
        "string",
        "number",
        "true",
        "false",
        "null",
        "template_string",
        "binary_expression",
        "unary_expression",
        "parenthesized_expression",
        "comma_expression",
        "formal_parameters",
        "type_annotation",
    ]
    .iter()
    .copied()
    .collect();

    let required_node_types: Vec<String> = pos_features[0]
        .node_types
        .iter()
        .filter(|nt| {
            !noise.contains(nt.as_str())
                && pos_features.iter().all(|f| f.node_types.contains(*nt))
                && !neg_nts.contains(&nt.as_str())
        })
        .cloned()
        .collect();

    let forbidden_node_types: Vec<String> = neg_features[0]
        .node_types
        .iter()
        .filter(|nt| {
            !noise.contains(nt.as_str())
                && neg_features.iter().all(|f| f.node_types.contains(*nt))
                && !pos_nts.contains(&nt.as_str())
        })
        .cloned()
        .collect();

    crate::corpus::semantic::LearnedConstraints {
        required_calls,
        forbidden_calls,
        required_node_types,
        forbidden_node_types,
        required_taint_flows: Vec::new(),
    }
}
