// SPDX-License-Identifier: MIT

use gensense::engine::auditor::GenSenseAuditor;
use gensense::engine::Engine;
use gensense::semantics::SymbolRegistry;
use std::path::Path;

#[test]
fn test_project_rule_must_have_guard() {
    let auditor = GenSenseAuditor::new(Vec::new());
    let _engine = Engine::new(auditor);

    // Add a project rule via YAML compilation simulation
    let yaml = r#"
project_rules:
  - id: GUARD_CHECK
    name: "Guard Check"
    severity: Critical
    observation: "Security guard check failed"
    category: Security
    impact: "Impact"
    improvement: "Improve"
    tags: ["security"]
    target_ext: "rs"
    must_have_guard:
      source_pattern: "handle_.*"
      guard_pattern: "check_auth"
      source_file_glob: "*"
      guard_file_glob: "*"
"#;

    // We can't easily mock the whole file system for Engine, but we can test the ProjectRuleIr directly.
    use gensense::rules::compiler::ProjectRuleCompiler;
    use gensense::rules::core::project::ProjectCoreRule;
    use gensense::semantics::symbols::{Symbol, SymbolKind};
    use gensense::ProjectRule;
    use gensense::SourceRegistry;

    let wrapper: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
    let p_rule_val = &wrapper["project_rules"][0];
    let p_rule_dsl: ProjectCoreRule = serde_yaml::from_value(p_rule_val.clone()).unwrap();
    let p_rule = ProjectRuleCompiler::compile(p_rule_dsl).expect("Failed to compile project rule");

    let mut symbols = SymbolRegistry::new();
    let mut sources = SourceRegistry::new();

    // File A: handler
    let handler = Symbol {
        name: "handle_request".to_string(),
        kind: SymbolKind::Function,
        start_byte: 0,
        end_byte: 10,
        file_path: "src/main.rs".to_string(),
        line: 1,
        column: 1,
        end_line: 5,
    };
    sources.register(
        Path::new("src/main.rs"),
        "fn handle_request() {}".to_string(),
    );
    let h_idx = symbols.insert(handler.clone());

    // File B: other func (no guard)
    let other = Symbol {
        name: "other_func".to_string(),
        kind: SymbolKind::Function,
        start_byte: 0,
        end_byte: 10,
        file_path: "src/lib.rs".to_string(),
        line: 1,
        column: 1,
        end_line: 5,
    };
    sources.register(Path::new("src/lib.rs"), "fn other_func() {}".to_string());
    let o_idx = symbols.insert(other.clone());

    // No edges yet. Should fail.
    let advisories = p_rule.check_project(&symbols, &sources);
    assert_eq!(advisories.len(), 1);
    assert!(advisories[0]
        .observation
        .contains("missing a reachable security guard"));

    // Add call edge to a non-guard
    symbols
        .graph
        .add_edge(h_idx, o_idx, gensense::semantics::graph::EdgeKind::Calls);
    let advisories = p_rule.check_project(&symbols, &sources);
    assert_eq!(advisories.len(), 1); // Still fails

    // Add a guard symbol and link it
    let guard = Symbol {
        name: "check_auth".to_string(),
        kind: SymbolKind::Function,
        start_byte: 0,
        end_byte: 10,
        file_path: "src/auth.rs".to_string(),
        line: 1,
        column: 1,
        end_line: 5,
    };
    sources.register(Path::new("src/auth.rs"), "fn check_auth() {}".to_string());
    let g_idx = symbols.insert(guard);

    // handle -> other -> guard
    symbols
        .graph
        .add_edge(o_idx, g_idx, gensense::semantics::graph::EdgeKind::Calls);

    let advisories = p_rule.check_project(&symbols, &sources);
    assert_eq!(advisories.len(), 0); // Passes now!
}

#[test]
fn test_project_rule_must_be_internal() {
    use gensense::rules::compiler::ProjectRuleCompiler;
    use gensense::rules::core::project::ProjectCoreRule;
    use gensense::semantics::symbols::{Symbol, SymbolKind};
    use gensense::ProjectRule;
    use gensense::SourceRegistry;

    let yaml = r#"
project_rules:
  - id: INTERNAL_CHECK
    name: "Internal Check"
    severity: Warning
    observation: "Internal logic leakage detected"
    category: Architecture
    impact: "Impact"
    improvement: "Improve"
    tags: ["arch"]
    target_ext: "rs"
    must_be_internal:
      pattern: "internal_.*"
      file_glob: "*"
"#;

    let wrapper: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
    let p_rule_val = &wrapper["project_rules"][0];
    let p_rule_dsl: ProjectCoreRule = serde_yaml::from_value(p_rule_val.clone()).unwrap();
    let p_rule = ProjectRuleCompiler::compile(p_rule_dsl).expect("Failed to compile project rule");

    let mut symbols = SymbolRegistry::new();
    let mut sources = SourceRegistry::new();

    let internal = Symbol {
        name: "internal_logic".to_string(),
        kind: SymbolKind::Function,
        start_byte: 0,
        end_byte: 10,
        file_path: "src/secret.rs".to_string(),
        line: 1,
        column: 1,
        end_line: 5,
    };
    sources.register(
        Path::new("src/secret.rs"),
        "fn internal_logic() {}".to_string(),
    );
    let i_idx = symbols.insert(internal);

    let external = Symbol {
        name: "public_api".to_string(),
        kind: SymbolKind::Function,
        start_byte: 0,
        end_byte: 10,
        file_path: "src/main.rs".to_string(),
        line: 1,
        column: 1,
        end_line: 5,
    };
    sources.register(Path::new("src/main.rs"), "fn public_api() {}".to_string());
    let e_idx = symbols.insert(external);

    // No calls. Passes.
    let advisories = p_rule.check_project(&symbols, &sources);
    assert_eq!(advisories.len(), 0);

    // Internal call (same file). Passes.
    let local_caller = Symbol {
        name: "local_helper".to_string(),
        kind: SymbolKind::Function,
        start_byte: 20,
        end_byte: 30,
        file_path: "src/secret.rs".to_string(),
        line: 10,
        column: 1,
        end_line: 15,
    };
    let l_idx = symbols.insert(local_caller);
    symbols
        .graph
        .add_edge(l_idx, i_idx, gensense::semantics::graph::EdgeKind::Calls);
    let advisories = p_rule.check_project(&symbols, &sources);
    assert_eq!(advisories.len(), 0);

    // External call (different file). Fails.
    symbols
        .graph
        .add_edge(e_idx, i_idx, gensense::semantics::graph::EdgeKind::Calls);
    let advisories = p_rule.check_project(&symbols, &sources);
    assert_eq!(advisories.len(), 1);
    assert!(advisories[0]
        .observation
        .contains("called from outside its file"));
}

#[test]
fn test_project_rule_cross_file_taint_free() {
    use gensense::rules::compiler::ProjectRuleCompiler;
    use gensense::rules::core::project::ProjectCoreRule;
    use gensense::semantics::symbols::{Symbol, SymbolKind};
    use gensense::ProjectRule;
    use gensense::SourceRegistry;

    let yaml = r#"
project_rules:
  - id: TAINT_CHECK
    name: "Taint Check"
    severity: Critical
    observation: "Taint flow detected"
    category: Security
    impact: "Impact"
    improvement: "Improve"
    tags: ["security"]
    target_ext: "rs"
    cross_file_taint_free:
      source_pattern: "req_.*"
      sink_pattern: "exec_sql"
"#;

    let wrapper: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
    let p_rule_val = &wrapper["project_rules"][0];
    let p_rule_dsl: ProjectCoreRule = serde_yaml::from_value(p_rule_val.clone()).unwrap();
    let p_rule = ProjectRuleCompiler::compile(p_rule_dsl).expect("Failed to compile project rule");

    let mut symbols = SymbolRegistry::new();
    let mut sources = SourceRegistry::new();

    let source = Symbol {
        name: "req_handler".to_string(),
        kind: SymbolKind::Function,
        start_byte: 0,
        end_byte: 10,
        file_path: "src/api.rs".to_string(),
        line: 1,
        column: 1,
        end_line: 5,
    };
    sources.register(Path::new("src/api.rs"), "fn req_handler() {}".to_string());
    let s_idx = symbols.insert(source);

    let mid = Symbol {
        name: "db_query".to_string(),
        kind: SymbolKind::Function,
        start_byte: 0,
        end_byte: 10,
        file_path: "src/db.rs".to_string(),
        line: 1,
        column: 1,
        end_line: 5,
    };
    sources.register(Path::new("src/db.rs"), "fn db_query() {}".to_string());
    let m_idx = symbols.insert(mid);

    let sink = Symbol {
        name: "exec_sql".to_string(),
        kind: SymbolKind::Function,
        start_byte: 0,
        end_byte: 10,
        file_path: "src/postgres.rs".to_string(),
        line: 1,
        column: 1,
        end_line: 5,
    };
    sources.register(Path::new("src/postgres.rs"), "fn exec_sql() {}".to_string());
    let snk_idx = symbols.insert(sink);

    // No path yet.
    let advisories = p_rule.check_project(&symbols, &sources);
    assert_eq!(advisories.len(), 0);

    // Add path: req -> db -> exec
    symbols
        .graph
        .add_edge(s_idx, m_idx, gensense::semantics::graph::EdgeKind::Calls);
    symbols
        .graph
        .add_edge(m_idx, snk_idx, gensense::semantics::graph::EdgeKind::Calls);

    let advisories = p_rule.check_project(&symbols, &sources);
    assert_eq!(advisories.len(), 1);
    assert!(advisories[0]
        .observation
        .contains("can reach sensitive sink"));
}

#[test]
fn test_bfs_does_not_deduplicate_across_files() {
    use gensense::rules::compiler::ProjectRuleCompiler;
    use gensense::rules::core::project::ProjectCoreRule;
    use gensense::semantics::symbols::{Symbol, SymbolKind};
    use gensense::ProjectRule;
    use gensense::SourceRegistry;
    // Let's use the pattern from the document.
    let yaml = r#"
project_rules:
  - id: GUARD_CHECK
    name: "Guard Check"
    severity: Critical
    observation: "Security guard check failed"
    category: Security
    impact: "Impact"
    improvement: "Improve"
    tags: ["security"]
    target_ext: "rs"
    must_have_guard:
      source_pattern: "handle_.*"
      guard_pattern: "auth_.*"
      source_file_glob: "*"
      guard_file_glob: "*"
"#;

    let wrapper: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
    let p_rule_val = &wrapper["project_rules"][0];
    let p_rule_dsl: ProjectCoreRule = serde_yaml::from_value(p_rule_val.clone()).unwrap();
    let _p_rule = ProjectRuleCompiler::compile(p_rule_dsl).unwrap();

    let mut symbols = SymbolRegistry::new();
    let mut sources = SourceRegistry::new();

    // 1. handler in api.rs
    let h_idx = symbols.insert(Symbol {
        name: "handle_request".to_string(),
        kind: SymbolKind::Function,
        start_byte: 0,
        end_byte: 10,
        file_path: "src/api.rs".to_string(),
        line: 1,
        column: 1,
        end_line: 5,
    });
    sources.register(
        Path::new("src/api.rs"),
        "fn handle_request() {}".to_string(),
    );

    // 2. "new" in db.rs (intermediate, same name as guard)
    let db_new_idx = symbols.insert(Symbol {
        name: "new".to_string(),
        kind: SymbolKind::Function,
        start_byte: 0,
        end_byte: 10,
        file_path: "src/db.rs".to_string(),
        line: 1,
        column: 1,
        end_line: 5,
    });
    sources.register(Path::new("src/db.rs"), "fn new() {}".to_string());

    // 3. "new" in auth.rs (the real guard, but wait, guard_pattern is auth_.*)
    // Let's change the test to match the doc's logic better.
    // If both are named "new", and the guard is the one in auth.rs.

    let auth_new_idx = symbols.insert(Symbol {
        name: "new".to_string(),
        kind: SymbolKind::Function,
        start_byte: 0,
        end_byte: 10,
        file_path: "src/auth.rs".to_string(),
        line: 1,
        column: 1,
        end_line: 5,
    });
    sources.register(Path::new("src/auth.rs"), "fn new() {}".to_string());

    // Update rule to look for "new" as guard, but only in auth.rs
    let yaml_v2 = r#"
project_rules:
  - id: GUARD_CHECK
    name: "Guard Check"
    severity: Critical
    observation: "Security guard check failed"
    category: Security
    impact: "Impact"
    improvement: "Improve"
    tags: ["security"]
    target_ext: "rs"
    must_have_guard:
      source_pattern: "handle_.*"
      guard_pattern: "new"
      source_file_glob: "*"
      guard_file_glob: "src/auth.rs"
"#;
    let wrapper_v2: serde_yaml::Value = serde_yaml::from_str(yaml_v2).unwrap();
    let p_rule_val_v2 = &wrapper_v2["project_rules"][0];
    let p_rule_dsl_v2: ProjectCoreRule = serde_yaml::from_value(p_rule_val_v2.clone()).unwrap();
    let p_rule_v2 = ProjectRuleCompiler::compile(p_rule_dsl_v2).unwrap();

    // Connect handler -> db::new -> auth::new
    symbols.graph.add_edge(
        h_idx,
        db_new_idx,
        gensense::semantics::graph::EdgeKind::Calls,
    );
    symbols.graph.add_edge(
        db_new_idx,
        auth_new_idx,
        gensense::semantics::graph::EdgeKind::Calls,
    );

    let advisories = p_rule_v2.check_project(&symbols, &sources);

    // With the bug:
    // BFS visits db::new, marks "new" visited.
    // BFS then looks at callees of db::new, which includes auth::new.
    // auth::new has name "new", which is already in visited.
    // BFS skips auth::new.
    // BFS finishes without finding a guard matching "new" in "src/auth.rs".
    // Advisory is generated (False Negative).

    // With the fix:
    // BFS visits ("new", "src/db.rs").
    // BFS then visits ("new", "src/auth.rs") because it's a different key.
    // auth::new matches the guard pattern AND the glob.
    // No advisory is generated.

    assert_eq!(
        advisories.len(),
        0,
        "BFS should have visited both 'new' functions and found the guard"
    );
}
