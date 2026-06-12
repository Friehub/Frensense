// SPDX-License-Identifier: MIT

use crate::ScopeId;
use std::collections::HashMap;
use std::sync::Arc;

/// Analysis Artifact: Represents a computed semantic result (e.g., Taint Graph)
pub trait AnalysisArtifact: Send + Sync {}

#[derive(Default)]
pub struct AnalysisRegistry {
    // We use a simplified registry for now, storing Arc<dyn Any> or specialized maps
    // For Frensense, we primarily care about Taint results and Symbol tables per scope.
    taint_results: HashMap<(String, ScopeId), Arc<dyn std::any::Any + Send + Sync>>,
}

impl AnalysisRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an error if the stored type `T` does not match the requested type.
    ///
    /// # Errors
    /// Returns an error if the type `T` does not match the stored type.
    pub fn get_or_compute<T, F>(
        &mut self,
        rule_id: &str,
        scope: ScopeId,
        compute: F,
    ) -> crate::Result<Arc<T>>
    where
        T: 'static + Send + Sync,
        F: FnOnce() -> T,
    {
        let key = (rule_id.to_string(), scope);
        let entry = self
            .taint_results
            .entry(key)
            .or_insert_with(|| Arc::new(compute()));

        entry.clone().downcast::<T>().map_err(|_| {
            crate::FrensenseError::Engine("Type mismatch in AnalysisRegistry".to_string())
        })
    }
}
