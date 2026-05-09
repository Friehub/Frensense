// SPDX-License-Identifier: MIT

pub mod lookup;
pub mod tracking;

use crate::GenSenseContext;
use std::collections::HashMap;
use tree_sitter::Node;

#[derive(Debug, Default, Clone)]
pub struct TaintRegistry {
    pub mappings: HashMap<String, String>, // var -> origin
}

impl TaintRegistry {
    pub fn taint(&mut self, var: String, origin: String) {
        self.mappings.insert(var, origin);
    }

    pub fn get_origin(&self, var: &str) -> Option<String> {
        self.mappings.get(var).cloned()
    }

    pub fn is_tainted(&self, var: &str) -> bool {
        self.mappings.contains_key(var)
    }
}

pub struct DataFlowAnalyzer<'a> {
    pub context: &'a GenSenseContext<'a>,
    pub root: Node<'a>,
    pub depth: usize,
    pub max_depth: usize,
}

impl<'a> DataFlowAnalyzer<'a> {
    pub fn new(context: &'a GenSenseContext<'a>, root: Node<'a>) -> Self {
        Self {
            context,
            root,
            depth: 0,
            max_depth: 5,
        }
    }

    pub fn with_depth(
        context: &'a GenSenseContext<'a>,
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
