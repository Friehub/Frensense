// SPDX-License-Identifier: MIT

pub mod handlers;
pub mod lookup;
pub mod normalization;
pub mod resolve;
pub mod tracking;

use crate::FileId;
use crate::FrensenseContext;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashSet;
use std::path::Path;
use tree_sitter::Node;

pub use frensense_engine::data_flow::TaintOrigin;
pub use frensense_engine::data_flow::TaintRegistry;

pub struct DataFlowAnalyzer<'a, 'ctx> {
    pub(crate) context: &'ctx FrensenseContext<'a>,
    pub(crate) current_source: &'a str,
    pub(crate) current_tree: &'a tree_sitter::Tree,
    pub(crate) current_file_path: &'a Path,
    pub(crate) current_file_id: FileId,
    pub(crate) root: Node<'a>,
    pub(crate) depth: usize,
    pub(crate) max_depth: usize,
    pub(crate) visited: RefCell<HashSet<(String, usize)>>,
    pub(crate) data_flow_engine: Option<&'ctx frensense_engine::data_flow::DataFlowEngine>,
    pub(crate) alias_tracker: RefCell<frensense_engine::data_flow::AliasTracker>,
    pub(crate) sanitize_re: Option<Regex>,
    pub(crate) seeder: Option<crate::engine::taint_seeder::TaintSeeder<'a>>,
}

impl<'a, 'ctx> DataFlowAnalyzer<'a, 'ctx> {
    #[must_use]
    pub fn new(context: &'ctx FrensenseContext<'a>, root: Node<'a>) -> Self {
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
            data_flow_engine: None,
            alias_tracker: RefCell::new(frensense_engine::data_flow::AliasTracker::new()),
            sanitize_re: None,
            seeder: None,
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

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn with_depth(
        context: &'ctx FrensenseContext<'a>,
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
            data_flow_engine: None,
            alias_tracker: RefCell::new(frensense_engine::data_flow::AliasTracker::new()),
            sanitize_re: None,
            seeder: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn with_depth_and_engine(
        context: &'ctx FrensenseContext<'a>,
        current_source: &'a str,
        current_tree: &'a tree_sitter::Tree,
        current_file_path: &'a Path,
        current_file_id: FileId,
        root: Node<'a>,
        depth: usize,
        max_depth: usize,
        engine: &'ctx frensense_engine::data_flow::DataFlowEngine,
        alias_tracker: frensense_engine::data_flow::AliasTracker,
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
            data_flow_engine: Some(engine),
            alias_tracker: RefCell::new(alias_tracker),
            sanitize_re: None,
            seeder: None,
        }
    }

    #[must_use]
    pub fn with_seeder(mut self, seeder: crate::engine::taint_seeder::TaintSeeder<'a>) -> Self {
        self.seeder = Some(seeder);
        self
    }
}
