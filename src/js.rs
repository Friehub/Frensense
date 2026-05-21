use crate::engine::Engine;
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
    pub enclosing_symbol: Option<String>,
    pub confidence: f64,
    pub fingerprint: String,
    pub auto_fixable: bool,
    pub requires_human: bool,
}

#[napi]
pub struct GenSenseEngine {
    inner: Engine,
}

#[napi]
impl GenSenseEngine {
    #[napi(constructor)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Engine::new(),
        }
    }

    #[napi(getter)]
    #[must_use]
    pub fn version(&self) -> String {
        crate::GENSENSE_VERSION.to_string()
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
    #[allow(clippy::needless_pass_by_value)]
    pub fn enable_tag(&mut self, tag: String) {
        self.inner.enable_tag(&tag);
    }

    #[napi]
    #[allow(clippy::needless_pass_by_value)]
    pub fn set_environment(&mut self, env: String) {
        let env_enum = match env.as_str() {
            "production" => crate::GenSenseEnvironment::Production,
            "staging" => crate::GenSenseEnvironment::Staging,
            _ => crate::GenSenseEnvironment::Development,
        };
        self.inner.set_environment(env_enum);
    }

    /// Analyse a single file in isolation. Per-file rules (style, security patterns,
    /// AI artifacts) run in full. Cross-file project rules (`MustHaveGuard`,
    /// `MustBeInternal`, `CrossFileTaintFree`) are NOT run — use `audit_project` for
    /// those.
    /// Audit code content directly.
    ///
    /// # Errors
    /// Returns an error if the engine fails to parse or scan the content.
    #[napi]
    #[allow(clippy::needless_pass_by_value)]
    pub fn audit_content(
        &mut self,
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
                    enclosing_symbol: a.enclosing_symbol,
                    confidence: f64::from(a.confidence),
                    fingerprint: a.fingerprint,
                    auto_fixable: a.auto_fixable,
                    requires_human: a.requires_human,
                })
                .collect()),
            Err(e) => Err(napi::Error::from_reason(format!(
                "GenSense Engine Error: {e}"
            ))),
        }
    }

    /// Audit an entire project directory, including cross-file project rules.
    /// Use this instead of `audit_content` when you need `MustHaveGuard`,
    /// `MustBeInternal`, or `CrossFileTaintFree` rules to run.
    ///
    /// # Errors
    /// Returns an error if the engine fails to access the project directory or scan the project.
    #[napi]
    #[allow(clippy::needless_pass_by_value)]
    pub fn audit_project(&mut self, root_dir: String) -> napi::Result<Vec<JsAdvisory>> {
        let root = Path::new(&root_dir);
        if !root.exists() {
            return Err(napi::Error::from_reason(format!(
                "Path does not exist: {root_dir}"
            )));
        }
        match self.inner.run(root) {
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
                    enclosing_symbol: a.enclosing_symbol,
                    confidence: f64::from(a.confidence),
                    fingerprint: a.fingerprint,
                    auto_fixable: a.auto_fixable,
                    requires_human: a.requires_human,
                })
                .collect()),
            Err(e) => Err(napi::Error::from_reason(format!(
                "GenSense Engine Error: {e}"
            ))),
        }
    }

    /// Audit a specific file or directory.
    ///
    /// # Errors
    /// Returns an error if the engine fails to access the path or scan the content.
    #[napi]
    #[allow(clippy::needless_pass_by_value)]
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
                    enclosing_symbol: a.enclosing_symbol,
                    confidence: f64::from(a.confidence),
                    fingerprint: a.fingerprint,
                    auto_fixable: a.auto_fixable,
                    requires_human: a.requires_human,
                })
                .collect()),
            Err(e) => Err(napi::Error::from_reason(format!(
                "GenSense Engine Error: {e}"
            ))),
        }
    }
}
