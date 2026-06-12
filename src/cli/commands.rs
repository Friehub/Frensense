#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate
)]
use crate::parser::ParserRegistry;
use crate::rules::compiler::RuleCompiler;
use crate::rules::core::CoreRule;
use crate::{Engine, FrensenseAuditor, FrensenseRule, Result};
use std::path::Path;

pub fn print_help() {
    println!("Frensense - Semantic Code Analysis Engine");
    println!("Version: {}", crate::FRENSENSE_VERSION);
    println!("Analyzes Rust, TypeScript, JavaScript, and YAML codebases for bugs,");
    println!("anti-patterns, security risks, and SQL drift — with AST-level precision.");
    println!();
    println!("Usage: frensense [path] [options]");
    println!();
    println!("Arguments:");
    println!("  path                File or directory to scan (default: current directory)");
    println!();
    println!("Analysis Options:");
    println!("  --language <lang>   Language filter: rust, typescript, javascript, yaml");
    println!("  --diff-only         Only scan files changed since the last git commit");
    println!("  --severity <level>  Minimum severity: critical, warning, info");
    println!("  --tag <name>        Enable optional diagnostic tag (e.g., sbom, governance)");
    println!("  --suite <name>      Rule precision tier: default, extended, all");
    println!("  --no-builtin-rules  Disable built-in rules (use only custom --rules-dir)");
    println!("  --rules-dir <dir>   Load custom rules from a directory");
    println!("  --disable-rule <id> Disable a specific rule by ID (repeatable)");
    println!("  --override-severity <RULE_ID>:<level>  Override severity for a rule");
    println!("                      Level: critical, warning, info (repeatable)");
    println!();
    println!("Confidence & Tuning:");
    println!("  --confidence <tier>      Preset: high (≥0.85), medium (≥0.60), low (≥0.30), any");
    println!("  --min-confidence <0-1>   Raw confidence threshold (default: 0.0)");
    println!("  --jaccard-threshold <0-1>  Similarity threshold for duplicate detection");
    println!("  --confidence-boost-rate <0-1>  Boost rate for overlapping findings");
    println!("  --confidence-boost-max <0-1>   Maximum confidence boost");
    println!("  --max-source-lines <N>   Limit source lines for analysis");
    println!("  --ngram-window <N>       Fingerprint n-gram window size");
    println!("  --min-ngram-count <N>    Minimum n-gram count threshold");
    println!("  --taint-conf-inter <0-1> Interprocedural taint confidence threshold");
    println!("  --taint-conf-intra <0-1> Intraprocedural taint confidence threshold");
    println!("  --taint-max-depth <N>    Maximum taint propagation depth");
    println!();
    println!("Output Options:");
    println!("  --json              Output findings as JSON");
    println!("  --sarif             Output findings in SARIF format");
    println!("  --strict            Exit with code 1 if any findings match filter");
    println!("  --emit-baseline <file>   Save current findings as a baseline");
    println!("  --compare-baseline <file>  Compare findings against a baseline");
    println!();
    println!("Style Profile (v0.4.0):");
    println!("  --learn-profile     Build a project style profile from current codebase");
    println!("  --check-profile     Check code against learned profile for style anomalies");
    println!(
        "  --profile-threshold <0-1>  Surprise threshold for anomaly detection (default: 0.7)"
    );
    println!("  --profile-stats     Display profile statistics");
    #[cfg(feature = "remediation")]
    println!("  --fix               Apply automated remediation (experimental)");
    #[cfg(feature = "remediation")]
    println!("  --diff              Show unified diff of proposed changes");
    println!();
    println!("Development:");
    println!("  --version           Display version and enabled features");
    println!("  --list-rules        List all active rules and their severities");
    println!("  --generate-docs     Generate RULES.md documentation file");
    println!("  --debug <file>      Dump anonymized AST for a source file");
    println!("  test-rule <rule.yml> --fixture <file> --expect-finding <id>");
    println!("                      Test a custom rule against a fixture file");
    println!("                      Optional: --expect-line <N>");
    println!();
    println!("Examples:");
    println!("  frensense                            Scan current directory");
    println!("  frensense src/                       Scan a specific directory");
    println!("  frensense main.rs                    Scan a single file");
    println!("  frensense --language rust .           Scan Rust files only");
    println!("  frensense --diff-only --strict        Check changed files, fail on any finding");
    println!("  frensense --json --suite extended     Export extended scan as JSON");
    println!("  frensense --disable-rule RUST_STD_OUTPUT .    Disable a specific rule");
    println!("  frensense --override-severity FILE_TOO_LONG:info .  Change rule severity");
    println!("  frensense --emit-baseline baseline.json   Save baseline");
    println!("  frensense --compare-baseline baseline.json  Check for regressions");
    println!();
    println!("Features Enabled:");
    #[cfg(feature = "rust")]
    println!("  [x] Rust Analysis");
    #[cfg(feature = "typescript")]
    println!("  [x] TypeScript/JS Analysis");
    #[cfg(feature = "fingerprinting")]
    println!("  [x] N-Gram Fingerprinting");
    #[cfg(feature = "remediation")]
    println!("  [x] Auto-Remediation");
}

pub fn print_version() {
    println!(
        "Frensense v{} - Semantic Code Analysis Engine",
        crate::FRENSENSE_VERSION
    );
    println!("Ship with confidence. Audit with insight.");
    println!("\nFeatures Enabled:");
    #[cfg(feature = "rust")]
    println!("  [x] Rust Analysis");
    #[cfg(feature = "typescript")]
    println!("  [x] TypeScript/JS Analysis");
    #[cfg(feature = "fingerprinting")]
    println!("  [x] N-Gram Fingerprinting");
    #[cfg(feature = "remediation")]
    println!("  [x] Auto-Remediation");
}

#[derive(serde::Deserialize)]
pub struct RulesWrapper {
    pub rules: Vec<CoreRule>,
}

pub fn handle_test_rule(args: &[String]) {
    if args.len() < 7 {
        eprintln!(
            "Usage: frensense test-rule <rule.yml> --fixture <file> --expect-finding <id> [--expect-line <N>]"
        );
        std::process::exit(1);
    }
    let rule_file_path = args[2].clone();

    let mut fixture = None;
    let mut expect_id = None;
    let mut expect_line = None;

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--fixture" => {
                if let Some(f) = args.get(i + 1) {
                    fixture = Some(f.clone());
                    i += 1;
                }
            }
            "--expect-finding" => {
                if let Some(id) = args.get(i + 1) {
                    expect_id = Some(id.clone());
                    i += 1;
                }
            }
            "--expect-line" => {
                if let Some(line) = args.get(i + 1) {
                    expect_line = line.parse::<u32>().ok();
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let fixture_path = fixture.expect("Missing --fixture argument");
    let expected_id = expect_id.expect("Missing --expect-finding argument");

    let rule_content = std::fs::read_to_string(&rule_file_path).expect("Failed to read rule file");

    let wrapper: RulesWrapper =
        serde_yaml::from_str(&rule_content).expect("Failed to parse YAML rules");
    let mut rules: Vec<Box<dyn FrensenseRule>> = Vec::new();
    for rule in wrapper.rules {
        match RuleCompiler::compile(rule) {
            Ok(compiled) => rules.push(Box::new(compiled)),
            Err(e) => {
                eprintln!("Error compiling rule: {e}");
                std::process::exit(1);
            }
        }
    }

    let mut engine = Engine::new();
    engine.set_rules(rules);
    engine.set_isolate_rules(true);

    let advisories = engine
        .run(Path::new(&fixture_path))
        .expect("Analysis failed");

    advisories
        .iter()
        .find(|a| a.rule_id == expected_id)
        .map_or_else(
            || {
                println!("[FAIL: Rule not triggered] Expected to find rule {expected_id}");
                std::process::exit(1);
            },
            |finding| {
                if let Some(expected_line) = expect_line
                    && finding.line != expected_line
                {
                    println!(
                        "[FAIL: Line mismatch] Expected finding on line {}, but found on line {}",
                        expected_line, finding.line
                    );
                    std::process::exit(1);
                }
                println!("[PASS]");
                std::process::exit(0);
            },
        );
}

pub fn handle_generate_docs() -> Result<()> {
    use std::fmt::Write;
    let engine = Engine::new();
    let _ = engine.list_rules();
    let mut doc = String::new();
    doc.push_str("# Frensense Rule Catalog\n\n");
    doc.push_str(
        "This catalog lists all semantic rules currently active in the Frensense engine.\n\n",
    );
    doc.push_str("| Rule ID | Severity | Category | Description |\n");
    doc.push_str("| :--- | :--- | :--- | :--- |\n");
    for rule in engine.auditor().rules() {
        let meta = rule.metadata();
        let _ = writeln!(
            doc,
            "| `{}` | {:?} | {} | {} |",
            rule.id(),
            meta.severity,
            meta.category,
            meta.impact
        );
    }
    std::fs::write("RULES.md", doc).expect("Failed to write RULES.md");
    println!("[SUCCESS] Generated RULES.md");
    std::process::exit(0);
}

pub fn handle_debug_ast(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);
    let content = std::fs::read_to_string(path).expect("Failed to read file");
    let mut parser = tree_sitter::Parser::new();
    let language = ParserRegistry::get_language(path).expect("Unsupported language");
    parser
        .set_language(&language)
        .expect("Failed to set language");
    let tree = parser.parse(&content, None).expect("Parse failure");
    println!("Anonymized AST for {file_path}:\n");
    println!("{}", tree.root_node().to_sexp());
    std::process::exit(0);
}

pub fn handle_list_rules() -> Result<()> {
    let (rules, _project_rules) = FrensenseAuditor::default_rules();
    let mut engine = Engine::new();
    engine.set_rules(rules);
    let catalog = engine.list_rules();
    println!("Frensense: Active Rules Catalog");
    println!("{:->100}", "");
    println!("{:<30} | {:<10} | Description", "Rule ID", "Severity");
    println!("{:->100}", "");
    for (id, name, sev) in catalog {
        println!("{id:<30} | {sev:<10} | {name}");
    }
    std::process::exit(0);
}

pub fn handle_early_args(args: &[String]) -> bool {
    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_help();
        std::process::exit(0);
    }

    if args.contains(&"--version".to_string()) {
        print_version();
        std::process::exit(0);
    }

    if args.len() > 1 && args[1] == "test-rule" {
        handle_test_rule(args);
        return true;
    }

    if args.contains(&"--generate-docs".to_string()) {
        if let Err(e) = handle_generate_docs() {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
        return true;
    }

    if let Some(pos) = args.iter().position(|a| a == "--debug")
        && let Some(file_path) = args.get(pos + 1)
    {
        if let Err(e) = handle_debug_ast(file_path) {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
        return true;
    }

    if args.contains(&"--list-rules".to_string()) {
        if let Err(e) = handle_list_rules() {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
        return true;
    }

    false
}
