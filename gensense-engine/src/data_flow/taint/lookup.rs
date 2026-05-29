// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::data_flow::TaintOrigin;

#[derive(Debug, Clone, Default)]
pub struct TaintLookup {
    taint_map: HashMap<String, TaintOrigin>,
    field_taint: HashMap<(String, String), TaintOrigin>,
    call_taint: HashMap<String, Vec<TaintOrigin>>,
}

impl TaintLookup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_tainted(&self, var: &str) -> bool {
        self.taint_map.contains_key(var) || self.field_taint.keys().any(|(v, _)| v == var)
    }

    pub fn get_origin(&self, var: &str) -> Option<TaintOrigin> {
        self.taint_map.get(var).cloned()
    }

    pub fn get_field_origin(&self, var: &str, field: &str) -> Option<TaintOrigin> {
        self.field_taint.get(&(var.to_string(), field.to_string())).cloned()
    }

    pub fn taint(&mut self, var: String, origin: TaintOrigin) {
        self.taint_map.insert(var, origin);
    }

    pub fn taint_field(&mut self, var: String, field: String, origin: TaintOrigin) {
        self.field_taint.insert((var, field), origin);
    }

    pub fn record_call_taint(&mut self, function_name: String, origin: TaintOrigin) {
        self.call_taint.entry(function_name).or_default().push(origin);
    }

    pub fn tainted_vars(&self) -> Vec<&str> {
        self.taint_map.keys().map(|s| s.as_str()).collect()
    }

    pub fn taint_sources_for_call(&self, function_name: &str) -> Vec<TaintOrigin> {
        self.call_taint.get(function_name).cloned().unwrap_or_default()
    }

    pub fn all_entries(&self) -> &HashMap<String, TaintOrigin> {
        &self.taint_map
    }

    pub fn clear(&mut self) {
        self.taint_map.clear();
        self.field_taint.clear();
        self.call_taint.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_taint() {
        let mut lookup = TaintLookup::new();
        lookup.taint("x".to_string(), TaintOrigin::UserInput);
        assert!(lookup.is_tainted("x"));
        assert_eq!(lookup.get_origin("x"), Some(TaintOrigin::UserInput));
        assert!(!lookup.is_tainted("y"));
    }

    #[test]
    fn test_field_taint() {
        let mut lookup = TaintLookup::new();
        lookup.taint_field("req".to_string(), "body".to_string(), TaintOrigin::UserInput);
        assert!(lookup.is_tainted("req"));
        assert_eq!(lookup.get_field_origin("req", "body"), Some(TaintOrigin::UserInput));
        assert_eq!(lookup.get_field_origin("req", "other"), None);
    }

    #[test]
    fn test_call_taint() {
        let mut lookup = TaintLookup::new();
        lookup.record_call_taint("get_user".to_string(), TaintOrigin::Database);
        let sources = lookup.taint_sources_for_call("get_user");
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0], TaintOrigin::Database);
    }
}
