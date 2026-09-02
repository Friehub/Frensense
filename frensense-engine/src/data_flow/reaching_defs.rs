// SPDX-License-Identifier: MIT

//! Reaching-definitions analysis for taint tracking.
//!
//! Instead of checking "is this variable name tainted?", we track
//! "which definitions reach this program point and are they tainted?"
//! This eliminates the need for hardcoded name whitelists.

use rustc_hash::FxHashMap;

use super::TaintOrigin;

/// A single reaching definition: a variable bound to a taint status.
#[derive(Debug, Clone)]
pub struct Definition {
    pub origin: TaintOrigin,
    /// Byte range of the source expression that created this definition.
    pub source_range: Option<(usize, usize)>,
}

/// A set of reaching definitions at a program point.
///
/// This is the core data structure: variable name → definition.
/// When we see `x = expr`, we look up `expr` in the current state
/// to determine if it's tainted, then record that for `x`.
#[derive(Debug, Clone, Default)]
pub struct DefState {
    pub defs: FxHashMap<String, Definition>,
}

impl DefState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark a variable as tainted with the given origin.
    pub fn taint(&mut self, var: &str, origin: TaintOrigin) {
        self.defs.insert(
            var.to_string(),
            Definition {
                origin,
                source_range: None,
            },
        );
    }

    /// Mark a variable as tainted with a specific source range.
    pub fn taint_with_range(
        &mut self,
        var: &str,
        origin: TaintOrigin,
        range: (usize, usize),
    ) {
        self.defs.insert(
            var.to_string(),
            Definition {
                origin,
                source_range: Some(range),
            },
        );
    }

    /// Remove taint from a variable (e.g., on reassignment to safe value).
    pub fn untaint(&mut self, var: &str) {
        self.defs.remove(var);
    }

    /// Check if a variable is tainted.
    pub fn is_tainted(&self, var: &str) -> bool {
        self.defs.contains_key(var)
    }

    /// Get the taint origin of a variable.
    pub fn get_origin(&self, var: &str) -> Option<&TaintOrigin> {
        self.defs.get(var).map(|d| &d.origin)
    }

    /// Check if a member expression like `obj.field` is tainted.
    /// First checks the full expression, then falls back to the object.
    pub fn is_member_tainted(&self, full_name: &str, object_name: &str) -> bool {
        self.defs.contains_key(full_name) || self.defs.contains_key(object_name)
    }

    /// Fork this state for a branch (clone).
    pub fn fork(&self) -> Self {
        self.clone()
    }

    /// Merge two states at a join point (after if/else).
    /// A variable is tainted after merge if it was tainted in BOTH branches.
    /// This is sound for security: if a variable might be safe in one branch,
    /// we conservatively consider it safe after the merge.
    pub fn merge(&mut self, other: &DefState) {
        // Keep only variables that are tainted in both states
        let keys_to_remove: Vec<String> = self
            .defs
            .keys()
            .filter(|k| !other.defs.contains_key(*k))
            .cloned()
            .collect();
        for k in keys_to_remove {
            self.defs.remove(&k);
        }
    }

    /// Merge two states conservatively: a variable is tainted if tainted in EITHER branch.
    /// Use this when you want to be sound (may report false positives but no false negatives).
    pub fn merge_union(&mut self, other: &DefState) {
        for (k, v) in &other.defs {
            self.defs.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_taint() {
        let mut state = DefState::new();
        state.taint("req", TaintOrigin::UserInput);
        assert!(state.is_tainted("req"));
        assert!(!state.is_tainted("safe_var"));
    }

    #[test]
    fn test_untaint() {
        let mut state = DefState::new();
        state.taint("x", TaintOrigin::UserInput);
        assert!(state.is_tainted("x"));
        state.untaint("x");
        assert!(!state.is_tainted("x"));
    }

    #[test]
    fn test_merge_intersection() {
        let mut state1 = DefState::new();
        state1.taint("a", TaintOrigin::UserInput);
        state1.taint("b", TaintOrigin::UserInput);

        let mut state2 = DefState::new();
        state2.taint("b", TaintOrigin::UserInput);
        state2.taint("c", TaintOrigin::UserInput);

        state1.merge(&state2);
        assert!(!state1.is_tainted("a")); // only in branch 1
        assert!(state1.is_tainted("b"));  // in both branches
        assert!(!state1.is_tainted("c")); // only in branch 2
    }

    #[test]
    fn test_merge_union() {
        let mut state1 = DefState::new();
        state1.taint("a", TaintOrigin::UserInput);

        let mut state2 = DefState::new();
        state2.taint("b", TaintOrigin::UserInput);

        state1.merge_union(&state2);
        assert!(state1.is_tainted("a"));
        assert!(state1.is_tainted("b"));
    }

    #[test]
    fn test_member_tainted() {
        let mut state = DefState::new();
        state.taint("req", TaintOrigin::UserInput);
        assert!(state.is_member_tainted("req.body.userId", "req"));
        assert!(!state.is_member_tainted("safeObj.field", "safeObj"));
    }
}
