// SPDX-License-Identifier: MIT

use crate::Advisory;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ConsistencyCheck {
    pub path_ast: Vec<Advisory>,   // Path B: AST -> Rule
    pub path_graph: Vec<Advisory>, // Path A: AST -> Graph -> Rule
}

#[derive(Debug, Clone)]
pub struct Divergence {
    pub missing_in_graph: Vec<Advisory>,
    pub extra_in_graph: Vec<Advisory>,
}

impl ConsistencyCheck {
    #[must_use]
    pub fn new(path_ast: Vec<Advisory>, path_graph: Vec<Advisory>) -> Self {
        Self {
            path_ast,
            path_graph,
        }
    }

    #[must_use]
    pub fn verify(&self) -> bool {
        if self.path_ast.len() != self.path_graph.len() {
            return false;
        }

        let set_a: HashSet<_> = self.path_ast.iter().collect();
        let set_b: HashSet<_> = self.path_graph.iter().collect();

        set_a == set_b
    }

    #[must_use]
    pub fn detect_divergence(&self) -> Divergence {
        let set_ast: HashSet<_> = self.path_ast.iter().collect();
        let set_graph: HashSet<_> = self.path_graph.iter().collect();

        let missing_in_graph = self
            .path_ast
            .iter()
            .filter(|a| !set_graph.contains(a))
            .cloned()
            .collect();

        let extra_in_graph = self
            .path_graph
            .iter()
            .filter(|a| !set_ast.contains(a))
            .cloned()
            .collect();

        Divergence {
            missing_in_graph,
            extra_in_graph,
        }
    }
}
