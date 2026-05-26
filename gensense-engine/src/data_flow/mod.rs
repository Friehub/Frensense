// SPDX-License-Identifier: MIT

pub mod normalization;

use std::collections::HashMap;
use tree_sitter::Node;

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
pub struct TaintRegistry<'a> {
    scopes: Vec<HashMap<&'a str, TaintOrigin>>,
    symbols: Vec<HashMap<&'a str, Node<'a>>>,
    field_taint: Vec<HashMap<(&'a str, &'a str), TaintOrigin>>,
}

impl Default for TaintRegistry<'_> {
    fn default() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            symbols: vec![HashMap::new()],
            field_taint: vec![HashMap::new()],
        }
    }
}

impl<'a> TaintRegistry<'a> {
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.symbols.push(HashMap::new());
        self.field_taint.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
            self.symbols.pop();
            self.field_taint.pop();
        }
    }

    pub fn taint_field(&mut self, var: &'a str, field: &'a str, origin: TaintOrigin) {
        if let Some(scope) = self.field_taint.last_mut() {
            scope.insert((var, field), origin);
        }
    }

    pub fn get_field_origin(&self, var: &str, field: &str) -> Option<TaintOrigin> {
        for scope in self.field_taint.iter().rev() {
            if let Some(origin) = scope.get(&(var, field)) {
                return Some(origin.clone());
            }
        }
        None
    }

    pub fn get_any_field_origin(&self, var: &str) -> Option<TaintOrigin> {
        for scope in self.field_taint.iter().rev() {
            for ((v, _), origin) in scope {
                if *v == var {
                    return Some(origin.clone());
                }
            }
        }
        None
    }

    pub fn taint(&mut self, var: &'a str, origin: TaintOrigin) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(var, origin);
        }
    }

    pub fn register_symbol(&mut self, name: &'a str, node: Node<'a>) {
        if let Some(scope) = self.symbols.last_mut() {
            scope.insert(name, node);
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

    pub fn find_symbol(&self, name: &str) -> Option<Node<'a>> {
        for scope in self.symbols.iter().rev() {
            if let Some(node) = scope.get(name) {
                return Some(*node);
            }
        }
        None
    }

    pub fn is_tainted(&self, var: &str) -> bool {
        self.get_origin(var).is_some() || self.get_any_field_origin(var).is_some()
    }
}
