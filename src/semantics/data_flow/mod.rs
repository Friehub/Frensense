// SPDX-License-Identifier: MIT

pub mod lookup;
pub mod normalization;
pub mod tracking;

#[cfg(test)]
mod equivalence_tests;

use crate::GenSenseContext;
use std::collections::HashMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct TaintRegistry<'a> {
    pub scopes: Vec<HashMap<&'a str, &'a str>>, // var -> origin
    pub symbols: Vec<HashMap<&'a str, Node<'a>>>, // var -> definition node
}

impl<'a> Default for TaintRegistry<'a> {
    fn default() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            symbols: vec![HashMap::new()],
        }
    }
}

impl<'a> TaintRegistry<'a> {
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.symbols.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
            self.symbols.pop();
        }
    }

    pub fn taint(&mut self, var: &'a str, origin: &'a str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(var, origin);
        }
    }

    pub fn register_symbol(&mut self, name: &'a str, node: Node<'a>) {
        if let Some(scope) = self.symbols.last_mut() {
            scope.insert(name, node);
        }
    }

    pub fn get_origin(&self, var: &str) -> Option<&'a str> {
        for scope in self.scopes.iter().rev() {
            if let Some(origin) = scope.get(var) {
                return Some(*origin);
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
        self.get_origin(var).is_some()
    }
}

pub struct DataFlowAnalyzer<'a, 'ctx> {
    pub context: &'ctx GenSenseContext<'a>,
    pub root: Node<'a>,
    pub depth: usize,
    pub max_depth: usize,
}

impl<'a, 'ctx> DataFlowAnalyzer<'a, 'ctx> {
    pub fn new(context: &'ctx GenSenseContext<'a>, root: Node<'a>) -> Self {
        Self {
            context,
            root,
            depth: 0,
            max_depth: 5,
        }
    }

    pub fn with_depth(
        context: &'ctx GenSenseContext<'a>,
        root: Node<'a>,
        depth: usize,
        max_depth: usize,
    ) -> Self {
        Self {
            context,
            root,
            depth,
            max_depth,
        }
    }
}
