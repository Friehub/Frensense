// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet};

use crate::data_flow::{TaintOrigin, TaintRegistry};

#[derive(Debug, Clone, Default)]
pub struct AliasTracker {
    aliases: HashMap<String, HashSet<String>>,
}

impl AliasTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_alias(&mut self, var: &str, target: &str) {
        if var == target {
            return;
        }

        let v = var.to_string();
        let t = target.to_string();

        let t_aliases: Vec<String> = self
            .aliases
            .get(&t)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default();

        let was_new = self.aliases.entry(v.clone()).or_default().insert(t.clone());
        if !was_new {
            return;
        }

        self.aliases.entry(v.clone()).or_default().extend(t_aliases.clone());

        let vars_to_update: Vec<String> = self
            .aliases
            .iter()
            .filter(|(other, targets)| *other != &v && targets.contains(&v))
            .map(|(other, _)| other.clone())
            .collect();

        for other in vars_to_update {
            let alias_set = self.aliases.entry(other).or_default();
            alias_set.insert(t.clone());
            alias_set.extend(t_aliases.clone());
        }
    }

    pub fn get_aliases(&self, var: &str) -> Vec<&str> {
        self.aliases
            .get(var)
            .map(|set| set.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    pub fn may_alias(&self, a: &str, b: &str) -> bool {
        if a == b {
            return true;
        }
        self.get_aliases(a).contains(&b) || self.get_aliases(b).contains(&a)
    }

    pub fn get_field_origin_with_aliases(
        &self,
        var: &str,
        field: &str,
        registry: &TaintRegistry,
    ) -> Option<TaintOrigin> {
        if let Some(origin) = registry.get_field_origin(var, field) {
            return Some(origin);
        }
        for alias in self.get_aliases(var) {
            if let Some(origin) = registry.get_field_origin(alias, field) {
                return Some(origin);
            }
        }
        None
    }

    pub fn get_origin_with_aliases(
        &self,
        var: &str,
        registry: &TaintRegistry,
    ) -> Option<TaintOrigin> {
        if let Some(origin) = registry.get_origin(var) {
            return Some(origin);
        }
        for alias in self.get_aliases(var) {
            if let Some(origin) = registry.get_origin(alias) {
                return Some(origin);
            }
        }
        None
    }

    pub fn is_tainted_with_aliases(&self, var: &str, registry: &TaintRegistry) -> bool {
        if registry.is_tainted(var) {
            return true;
        }
        for alias in self.get_aliases(var) {
            if registry.is_tainted(alias) {
                return true;
            }
        }
        false
    }

    pub fn clear(&mut self) {
        self.aliases.clear();
    }

    pub fn merge(&mut self, other: &Self) {
        for (var, targets) in &other.aliases {
            for target in targets {
                self.record_alias(var, target);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_and_query_alias() {
        let mut tracker = AliasTracker::new();
        tracker.record_alias("y", "x");
        assert!(tracker.may_alias("y", "x"));
        assert!(tracker.may_alias("x", "y"));
    }

    #[test]
    fn test_transitive_aliasing() {
        let mut tracker = AliasTracker::new();
        tracker.record_alias("z", "y");
        tracker.record_alias("y", "x");
        assert!(tracker.may_alias("z", "x"));
    }

    #[test]
    fn test_self_alias_not_recorded() {
        let mut tracker = AliasTracker::new();
        tracker.record_alias("x", "x");
        assert!(tracker.get_aliases("x").is_empty());
    }

    #[test]
    fn test_field_origin_with_aliases() {
        let mut registry = TaintRegistry::default();
        registry.taint_field("req", "body", TaintOrigin::UserInput);

        let mut tracker = AliasTracker::new();
        tracker.record_alias("r2", "req");

        assert_eq!(
            tracker.get_field_origin_with_aliases("r2", "body", &registry),
            Some(TaintOrigin::UserInput)
        );
        assert_eq!(
            tracker.get_field_origin_with_aliases("req", "body", &registry),
            Some(TaintOrigin::UserInput)
        );
        assert_eq!(
            tracker.get_field_origin_with_aliases("unknown", "body", &registry),
            None
        );
    }

    #[test]
    fn test_origin_with_aliases() {
        let mut registry = TaintRegistry::default();
        registry.taint("req", TaintOrigin::UserInput);

        let mut tracker = AliasTracker::new();
        tracker.record_alias("r2", "req");

        assert_eq!(
            tracker.get_origin_with_aliases("r2", &registry),
            Some(TaintOrigin::UserInput)
        );
        assert_eq!(
            tracker.get_origin_with_aliases("unknown", &registry),
            None
        );
    }
}
