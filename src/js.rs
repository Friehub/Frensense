use crate::engine::Engine;
use crate::GenSenseAuditor;
use napi_derive::napi;
use std::path::Path;

#[napi(object)]
#[derive(Clone)]
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
pub struct GenSenseEngine {
    inner: Engine,
}

#[napi]
impl GenSenseEngine {
    #[napi(constructor)]
    pub fn new() -> Self {
        let auditor = GenSenseAuditor::default_auditor();
        Self {
            inner: Engine::new(auditor),
        }
    }
}

impl Default for GenSenseEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
impl GenSenseEngine {
    #[napi]
    pub fn enable_tag(&mut self, tag: String) {
        self.inner.enable_tag(&tag);
    }

    #[napi]
    pub fn set_environment(&mut self, env: String) {
        let env_enum = match env.as_str() {
            "production" => crate::GenSenseEnvironment::Production,
            "staging" => crate::GenSenseEnvironment::Staging,
            _ => crate::GenSenseEnvironment::Development,
        };
        self.inner.set_environment(env_enum);
    }

    /// Analyse a single file in isolation. Per-file rules (style, security patterns,
    /// AI artifacts) run in full. Cross-file project rules (MustHaveGuard,
    /// MustBeInternal, CrossFileTaintFree) are NOT run — use `audit_project` for
    /// those.
    #[napi]
    pub fn audit_content(
        &self,
        file_path: String,
        content: String,
    ) -> napi::Result<Vec<JsAdvisory>> {
        match self.inner.run_content(Path::new(&file_path), &content) {
            Ok(advisories) => Ok(advisories
                .into_iter()
                .map(|a| JsAdvisory {
                    rule_id: a.rule_id,
                    severity: format!("{:?}", a.severity),
                    observation: a.observation,
                    impact: a.impact,
                    improvement: a.improvement,
                    line: a.line,
                    column: a.column,
                    file_path: a.file_path,
                })
                .collect()),
            Err(e) => Err(napi::Error::from_reason(format!(
                "GenSense Engine Error: {e}"
            ))),
        }
    }

    /// Audit an entire project directory, including cross-file project rules.
    /// Use this instead of `audit_content` when you need MustHaveGuard,
    /// MustBeInternal, or CrossFileTaintFree rules to run.
    #[napi]
    pub fn audit_project(&mut self, root_dir: String) -> napi::Result<Vec<JsAdvisory>> {
        match self.inner.run(Path::new(&root_dir)) {
            Ok(advisories) => Ok(advisories
                .into_iter()
                .map(|a| JsAdvisory {
                    rule_id: a.rule_id,
                    severity: format!("{:?}", a.severity),
                    observation: a.observation,
                    impact: a.impact,
                    improvement: a.improvement,
                    line: a.line,
                    column: a.column,
                    file_path: a.file_path,
                })
                .collect()),
            Err(e) => Err(napi::Error::from_reason(format!(
                "GenSense Engine Error: {e}"
            ))),
        }
    }

    #[napi]
    pub fn audit_path(&mut self, path: String) -> napi::Result<Vec<JsAdvisory>> {
        match self.inner.run(Path::new(&path)) {
            Ok(advisories) => Ok(advisories
                .into_iter()
                .map(|a| JsAdvisory {
                    rule_id: a.rule_id,
                    severity: format!("{:?}", a.severity),
                    observation: a.observation,
                    impact: a.impact,
                    improvement: a.improvement,
                    line: a.line,
                    column: a.column,
                    file_path: a.file_path,
                })
                .collect()),
            Err(e) => Err(napi::Error::from_reason(format!(
                "GenSense Engine Error: {e}"
            ))),
        }
    }
}
