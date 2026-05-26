// SPDX-License-Identifier: MIT

pub mod checks;
pub mod core;
pub mod flow;
pub mod project;

pub use core::{AstQuery, CoreRuleIr};
pub use flow::{FlowConstraint, FlowEvaluator, TemporalBehavior};
pub use project::{ProjectFlowConstraint, ProjectRuleIr, SchemaExtract, SchemaType};

pub(crate) fn find_project_root(sources: &crate::SourceRegistry) -> std::path::PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        return cwd;
    }

    let mut files = sources.all_files().peekable();
    if files.peek().is_none() {
        return std::path::PathBuf::from(".");
    }

    let mut common_path: Option<std::path::PathBuf> = None;
    for file in files {
        if let Some(ref current) = common_path {
            let mut common = std::path::PathBuf::new();
            let current_comps: Vec<_> = current.components().collect();
            let file_comps: Vec<_> = file.path.components().collect();
            for (c1, c2) in current_comps.into_iter().zip(file_comps) {
                if c1 == c2 {
                    common.push(c1);
                } else {
                    break;
                }
            }
            common_path = Some(common);
        } else {
            common_path = Some(file.path.parent().map_or_else(
                || std::path::PathBuf::from("."),
                std::path::Path::to_path_buf,
            ));
        }
    }

    common_path.unwrap_or_else(|| std::path::PathBuf::from("."))
}
