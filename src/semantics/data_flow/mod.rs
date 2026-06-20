// SPDX-License-Identifier: MIT

pub mod corpus_seeder;
pub mod cross_file;
pub mod handlers;
pub mod interprocedural;
pub mod lookup;
pub mod normalization;
pub mod tracking;

use crate::FrensenseContext;
use regex::Regex;
use std::cell::RefCell;
use std::path::Path;
use tree_sitter::Node;

pub use frensense_engine::data_flow::TaintOrigin;
pub use frensense_engine::data_flow::TaintRegistry;

pub struct DataFlowAnalyzer<'a, 'ctx> {
    pub(crate) context: &'ctx FrensenseContext<'a>,
    pub(crate) current_source: &'a str,
    pub(crate) current_tree: &'a tree_sitter::Tree,
    pub(crate) current_file_path: &'a Path,
    pub(crate) root: Node<'a>,
    pub(crate) data_flow_engine: Option<&'ctx frensense_engine::data_flow::DataFlowEngine>,
    pub(crate) alias_tracker: RefCell<frensense_engine::data_flow::AliasTracker>,
    pub(crate) sanitize_re: Option<Regex>,
}

impl<'a, 'ctx> DataFlowAnalyzer<'a, 'ctx> {
    #[must_use]
    pub fn new(context: &'ctx FrensenseContext<'a>, root: Node<'a>) -> Self {
        Self {
            context,
            current_source: context.source_code,
            current_tree: context.tree,
            current_file_path: context.file_path,
            root,
            data_flow_engine: None,
            alias_tracker: RefCell::new(frensense_engine::data_flow::AliasTracker::new()),
            sanitize_re: None,
        }
    }

    #[must_use]
    pub fn with_engine(
        mut self,
        engine: &'ctx frensense_engine::data_flow::DataFlowEngine,
    ) -> Self {
        self.data_flow_engine = Some(engine);
        self
    }

    #[must_use]
    pub fn with_alias_tracker(
        mut self,
        tracker: frensense_engine::data_flow::AliasTracker,
    ) -> Self {
        self.alias_tracker = RefCell::new(tracker);
        self
    }

    #[must_use]
    pub fn with_sanitizers(mut self, sanitize_re: Regex) -> Self {
        self.sanitize_re = Some(sanitize_re);
        self
    }
}
