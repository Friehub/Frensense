// SPDX-License-Identifier: MIT

pub mod handlers;
pub mod lookup;
pub mod normalization;
pub mod resolve;
pub mod tracking;

use crate::FileId;
use crate::GenSenseContext;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::Path;
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
pub struct TaintRegistry {
    scopes: Vec<HashMap<String, TaintOrigin>>, // var -> origin
    symbol_ranges: Vec<HashMap<String, (usize, usize)>>, // var -> (start_byte, end_byte)
    field_taint: Vec<HashMap<(String, String), TaintOrigin>>, // (var, field) -> origin
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

    #[must_use]
    pub fn get_field_origin(&self, var: &str, field: &str) -> Option<TaintOrigin> {
        let key = (var.to_string(), field.to_string());
        for scope in self.field_taint.iter().rev() {
            if let Some(origin) = scope.get(&key) {
                return Some(origin.clone());
            }
        }
        None
    }

    /// Returns the taint origin for any field of `var`, if at least one field is tainted.
    #[must_use]
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

    #[must_use]
    pub fn get_origin(&self, var: &str) -> Option<TaintOrigin> {
        for scope in self.scopes.iter().rev() {
            if let Some(origin) = scope.get(var) {
                return Some(origin.clone());
            }
        }
        None
    }

    #[must_use]
    pub fn find_symbol_range(&self, name: &str) -> Option<(usize, usize)> {
        for scope in self.symbol_ranges.iter().rev() {
            if let Some(range) = scope.get(name) {
                return Some(*range);
            }
        }
        None
    }

    #[must_use]
    pub fn is_tainted(&self, var: &str) -> bool {
        self.get_origin(var).is_some() || self.get_any_field_origin(var).is_some()
    }
}

pub struct DataFlowAnalyzer<'a, 'ctx> {
    pub(crate) context: &'ctx GenSenseContext<'a>,
    pub(crate) current_source: &'a str,
    pub(crate) current_tree: &'a tree_sitter::Tree,
    pub(crate) current_file_path: &'a Path,
    pub(crate) current_file_id: FileId,
    pub(crate) root: Node<'a>,
    pub(crate) depth: usize,
    pub(crate) max_depth: usize,
    pub(crate) visited: RefCell<HashSet<(String, usize)>>,
}

impl<'a, 'ctx> DataFlowAnalyzer<'a, 'ctx> {
    #[must_use]
    pub fn new(context: &'ctx GenSenseContext<'a>, root: Node<'a>) -> Self {
        Self {
            context,
            current_source: context.source_code,
            current_tree: context.tree,
            current_file_path: context.file_path,
            current_file_id: context.file_id,
            root,
            depth: 0,
            max_depth: context.default_taint_max_depth,
            visited: RefCell::new(HashSet::new()),
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn with_depth(
        context: &'ctx GenSenseContext<'a>,
        current_source: &'a str,
        current_tree: &'a tree_sitter::Tree,
        current_file_path: &'a Path,
        current_file_id: FileId,
        root: Node<'a>,
        depth: usize,
        max_depth: usize,
    ) -> Self {
        Self {
            context,
            current_source,
            current_tree,
            current_file_path,
            current_file_id,
            root,
            depth,
            max_depth,
            visited: RefCell::new(HashSet::new()),
        }
    }
}
