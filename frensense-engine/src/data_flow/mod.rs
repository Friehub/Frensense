// SPDX-License-Identifier: MIT

pub mod alias;
pub mod confidence;
pub mod cross_file;
pub mod engine;
pub mod normalization;
pub mod path_sensitive;
pub mod resolver;
pub mod taint_metrics;

pub use alias::AliasTracker;
pub use engine::DataFlowEngine;
pub use engine::FunctionTaintSummary;
pub use resolver::ResolvedFunction;
pub use resolver::SymbolEntry;
pub use resolver::map_call_args_to_params;
pub use resolver::resolve_fn_definition;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaintOrigin {
    UserInput,
    Environment,
    Database,
    Network,
    FileSystem,
    Custom(String),
}

impl std::fmt::Display for TaintOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserInput => write!(f, "user_input"),
            Self::Environment => write!(f, "environment"),
            Self::Database => write!(f, "database"),
            Self::Network => write!(f, "network"),
            Self::FileSystem => write!(f, "file_system"),
            Self::Custom(s) => write!(f, "{s}"),
        }
    }
}

impl From<&str> for TaintOrigin {
    fn from(s: &str) -> Self {
        match s {
            "user_input" | "user" => Self::UserInput,
            "environment" | "env" => Self::Environment,
            "database" | "db" => Self::Database,
            "network" | "net" => Self::Network,
            "file_system" | "fs" => Self::FileSystem,
            _ => Self::Custom(s.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaintRegistry {
    scopes: Vec<HashMap<String, TaintOrigin>>,
    symbol_ranges: Vec<HashMap<String, (usize, usize)>>,
    field_taint: Vec<HashMap<(String, String), TaintOrigin>>,
}

impl Default for TaintRegistry {
    fn default() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            symbol_ranges: vec![HashMap::new()],
            field_taint: vec![HashMap::new()],
        }
    }
}

impl TaintRegistry {
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.symbol_ranges.push(HashMap::new());
        self.field_taint.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
            self.symbol_ranges.pop();
            self.field_taint.pop();
        }
    }

    pub fn taint_field(&mut self, var: &str, field: &str, origin: TaintOrigin) {
        if let Some(scope) = self.field_taint.last_mut() {
            scope.insert((var.to_string(), field.to_string()), origin);
        }
    }

    pub fn get_field_origin(&self, var: &str, field: &str) -> Option<TaintOrigin> {
        let key = (var.to_string(), field.to_string());
        for scope in self.field_taint.iter().rev() {
            if let Some(origin) = scope.get(&key) {
                return Some(origin.clone());
            }
        }
        None
    }

    pub fn get_any_field_origin(&self, var: &str) -> Option<TaintOrigin> {
        for scope in self.field_taint.iter().rev() {
            for ((v, _), origin) in scope {
                if v == var {
                    return Some(origin.clone());
                }
            }
        }
        None
    }

    pub fn taint(&mut self, var: &str, origin: TaintOrigin) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(var.to_string(), origin);
        }
    }

    pub fn register_symbol(&mut self, name: &str, start_byte: usize, end_byte: usize) {
        if let Some(scope) = self.symbol_ranges.last_mut() {
            scope.insert(name.to_string(), (start_byte, end_byte));
        }
    }

    pub fn get_origin(&self, var: &str) -> Option<TaintOrigin> {
        for scope in self.scopes.iter().rev() {
            if let Some(origin) = scope.get(var) {
                return Some(origin.clone());
            }
        }
        None
    }

    pub fn find_symbol_range(&self, name: &str) -> Option<(usize, usize)> {
        for scope in self.symbol_ranges.iter().rev() {
            if let Some(range) = scope.get(name) {
                return Some(*range);
            }
        }
        None
    }

    pub fn is_tainted(&self, var: &str) -> bool {
        self.get_origin(var).is_some() || self.get_any_field_origin(var).is_some()
    }
}
