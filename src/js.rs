use crate::engine::Engine;
use crate::GenSenseAuditor;
use crate::SymbolRegistry;
use napi_derive::napi;
use std::path::Path;

#[napi(object)]
pub struct JsAdvisory {
    pub rule_id: String,
    pub severity: String,
    pub observation: String,
    pub impact: String,
    pub improvement: String,
    pub line: u32,
    pub column: u32,
    pub file_path: String,
}

#[napi]
pub fn audit_content(file_path: String, content: String) -> Vec<JsAdvisory> {
    let auditor = GenSenseAuditor::default_auditor();
    let symbols = SymbolRegistry::new();
    let engine = Engine::new(auditor);

    match engine.auditor.audit(
        Path::new(&file_path),
        &content,
        &symbols,
        &engine.enabled_categories,
        &engine.enabled_tags,
        engine.environment,
    ) {
        Ok((advisories, _)) => advisories
            .into_iter()
            .map(|a| JsAdvisory {
                rule_id: a.rule_id,
                severity: format!("{:?}", a.severity),
                observation: a.observation,
                impact: a.impact,
                improvement: a.improvement,
                line: a.line as u32,
                column: a.column as u32,
                file_path: a.file_path,
            })
            .collect(),
        Err(_e) => {
            vec![]
        }
    }
}
