#[cfg(test)]
mod tests {
    use std::path::Path;
    use taas_auditor::{AstAuditor, SymbolRegistry};

    #[test]
    fn test_rust_async_safety() {
        let auditor = AstAuditor::default_auditor();
        let code = r#"
            async fn bad() {
                let m = std::sync::Mutex::new(0);
                let _g = m.lock().unwrap();
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        "#;

        let registry = SymbolRegistry::new();
        let (violations, _) = auditor
            .audit(
                Path::new("test.rs"),
                code,
                &registry,
                &Default::default(),
                &Default::default(),
                taas_auditor::AuditorEnvironment::Development,
            )
            .unwrap();
        assert!(!violations.is_empty());

        let has_deadlock = violations
            .iter()
            .any(|v| v.rule_id == "RUST_ASYNC_MUTEX_DEADLOCK");
        assert!(has_deadlock, "Should detect Mutex deadlock");
    }

    #[test]
    fn test_js_security() {
        let auditor = AstAuditor::default_auditor();
        let code = "eval('console.log(1)');";

        let registry = SymbolRegistry::new();
        let (violations, _) = auditor
            .audit(
                Path::new("plugin.js"),
                code,
                &registry,
                &Default::default(),
                &Default::default(),
                taas_auditor::AuditorEnvironment::Development,
            )
            .unwrap();
        // The rule ID might have changed in YAML
        assert!(violations
            .iter()
            .any(|v| v.rule_id == "JS_DYNAMIC_EXECUTION"));
    }
}
