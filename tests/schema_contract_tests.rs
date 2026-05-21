// SPDX-License-Identifier: MIT

use gensense::SourceRegistry;
use gensense::engine::auditor::GenSenseAuditor;
use gensense::rules::compiler::ProjectRuleCompiler;
use gensense::rules::core::project::ProjectCoreRule;
use gensense::rules::schema_contract::prisma_extractor::PrismaExtractor;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_prisma_extractor_parses_models_fields_and_enums() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    let schema_dir = root.join("prisma");
    fs::create_dir_all(&schema_dir).unwrap();

    fs::write(
        schema_dir.join("schema.prisma"),
        r#"
        generator client {
          provider = "prisma-client-js"
        }

        model User {
          id String @id
          email String
          posts Post[]
          @@index([email])
        }

        model Post {
          id String @id
          userId String
        }

        enum Role {
          USER
          ADMIN @map("admin")
        }
        "#,
    )
    .unwrap();

    let schema_glob = glob::Pattern::new("**/*.prisma").unwrap();
    let models = PrismaExtractor::extract_model_names(&schema_glob, root);
    let fields = PrismaExtractor::extract_field_names(&schema_glob, root);
    let enums = PrismaExtractor::extract_enum_values(&schema_glob, root);

    assert!(models.contains("User"));
    assert!(models.contains("Post"));
    assert!(fields.contains("id"));
    assert!(fields.contains("email"));
    assert!(fields.contains("posts"));
    assert!(fields.contains("userId"));
    assert!(enums.contains("USER"));
    assert!(enums.contains("ADMIN"));
}

#[test]
fn test_schema_contract_rules_fire_from_schema_contracts_key() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join("prisma")).unwrap();
    fs::create_dir_all(root.join(".gensense").join("rules")).unwrap();

    fs::write(
        root.join("prisma").join("schema.prisma"),
        r#"
        model User {
          id String @id
          email String
        }
        "#,
    )
    .unwrap();

    fs::write(
        root.join("table.rs"),
        r#"
        fn table_query() {
            let sql = "SELECT * FROM MissingTable";
            let _ = sql;
        }
        "#,
    )
    .unwrap();

    fs::write(
        root.join("column.rs"),
        r##"
        fn column_query() {
            let sql = r#"SELECT * FROM "User" WHERE "badColumn" = $1"#;
            let _ = sql;
        }
        "##,
    )
    .unwrap();

    fs::write(
        root.join(".gensense").join("rules").join("schema.yml"),
        r#"
        schema_contracts:
          - id: RUST_SQL_TABLE_MUST_EXIST_IN_PRISMA
            name: Rust SQL Table Must Exist In Prisma
            severity: Critical
            observation: SQL table references must exist in Prisma models.
            category: Security
            impact: Raw SQL drift can break queries at runtime or target the wrong table.
            improvement: Rename the query table or update the Prisma model to match.
            tags: [database, prisma]
            source_ext: rs
            source_pattern: '(?:FROM|JOIN|INTO|UPDATE)\s+"?([A-Z][a-zA-Z0-9]+)"?'
            source_file_glob: '**/*.rs'
            schema_type: prisma
            schema_glob: '**/*.prisma'
            schema_extract: model_names

          - id: RUST_SQL_COLUMN_MUST_EXIST_IN_PRISMA
            name: Rust SQL Column Must Exist In Prisma
            severity: Critical
            observation: SQL column references must exist in Prisma fields.
            category: Security
            impact: Raw SQL drift can break queries at runtime or target the wrong column.
            improvement: Rename the column reference or update the Prisma field to match.
            tags: [database, prisma]
            source_ext: rs
            source_pattern: '"([a-z][a-zA-Z0-9]+)"\s*(?:[=<>!]|IS\s|IN\s)'
            source_file_glob: '**/*.rs'
            schema_type: prisma
            schema_glob: '**/*.prisma'
            schema_extract: field_names
        "#,
    )
    .unwrap();

    let (_, project_rules) = GenSenseAuditor::build_rule_set(root, &[], true);
    let mut sources = SourceRegistry::new();
    sources.register(
        &root.join("table.rs"),
        fs::read_to_string(root.join("table.rs")).unwrap(),
    );
    sources.register(
        &root.join("column.rs"),
        fs::read_to_string(root.join("column.rs")).unwrap(),
    );
    sources.register(
        &root.join("prisma").join("schema.prisma"),
        fs::read_to_string(root.join("prisma").join("schema.prisma")).unwrap(),
    );

    let advisories: Vec<_> = project_rules
        .iter()
        .flat_map(|rule| rule.check_project(&gensense::semantics::SymbolRegistry::new(), &sources))
        .collect();

    assert!(
        advisories
            .iter()
            .any(|a| a.rule_id == "RUST_SQL_TABLE_MUST_EXIST_IN_PRISMA"),
        "table contract should fire"
    );
    assert!(
        advisories
            .iter()
            .any(|a| a.rule_id == "RUST_SQL_COLUMN_MUST_EXIST_IN_PRISMA"),
        "column contract should fire"
    );
}

#[test]
fn test_project_rule_compiler_accepts_schema_contract_fields() {
    let yaml = r#"
    schema_contracts:
      - id: SCHEMA_RULE
        name: Schema Rule
        severity: Critical
        observation: Schema contract
        category: Security
        impact: Impact
        improvement: Improve
        tags: []
        source_ext: rs
        source_pattern: 'SELECT\s+([A-Za-z_][A-Za-z0-9_]*)'
        source_file_glob: '**/*.rs'
        schema_type: prisma
        schema_glob: '**/*.prisma'
        schema_extract: field_names
    "#;

    let wrapper: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
    let rule_value = &wrapper["schema_contracts"][0];
    let dsl: ProjectCoreRule = serde_yaml::from_value(rule_value.clone()).unwrap();
    let _compiled = ProjectRuleCompiler::compile(dsl).unwrap();
}

#[test]
fn test_schema_contract_source_registry_lookup_handles_relative_paths() {
    let mut sources = SourceRegistry::new();
    sources.register(
        Path::new("src/query.rs"),
        "let sql = \"SELECT x FROM Missing\";".to_string(),
    );
    assert!(sources.get_by_path(Path::new("src/query.rs")).is_some());
}
