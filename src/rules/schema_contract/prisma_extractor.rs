// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

pub struct PrismaExtractor;

impl PrismaExtractor {
    fn matching_schema_files(schema_glob: &glob::Pattern, root: &Path) -> Vec<std::path::PathBuf> {
        let mut files = Vec::new();
        let options = glob::MatchOptions {
            case_sensitive: true,
            require_literal_separator: true,
            require_literal_leading_dot: false,
        };

        for entry in WalkDir::new(root).into_iter().flatten() {
            if !entry.file_type().is_file() {
                continue;
            }

            let file_path = entry.path();
            if let Ok(rel_path) = file_path.strip_prefix(root) {
                if schema_glob.matches_with(rel_path.to_str().unwrap_or(""), options) {
                    files.push(file_path.to_path_buf());
                }
            }
        }

        files
    }

    fn parse_schema_file<FModel, FField, FEnum>(
        content: &str,
        mut on_model: FModel,
        mut on_field: FField,
        mut on_enum_value: FEnum,
    ) where
        FModel: FnMut(&str),
        FField: FnMut(&str),
        FEnum: FnMut(&str),
    {
        enum BlockKind {
            Model,
            Enum,
        }

        let mut active_block: Option<BlockKind> = None;
        let mut pending_block: Option<BlockKind> = None;

        for raw_line in content.lines() {
            let line = raw_line.trim();

            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            if let Some(kind) = active_block.as_ref() {
                if line.starts_with('}') {
                    active_block = None;
                    continue;
                }

                if line.starts_with("@@") || line.starts_with('@') {
                    continue;
                }

                let token = line.split_whitespace().next().unwrap_or("");
                if token.is_empty() {
                    continue;
                }

                match kind {
                    BlockKind::Model => {
                        if token.chars().all(|c| c.is_alphanumeric() || c == '_') {
                            on_field(token);
                        }
                    }
                    BlockKind::Enum => {
                        if token.chars().all(|c| c.is_alphanumeric() || c == '_') {
                            on_enum_value(token);
                        }
                    }
                }
                continue;
            }

            if let Some(kind) = pending_block.take() {
                if line.starts_with('{') || line.contains('{') {
                    active_block = Some(kind);
                } else {
                    pending_block = Some(kind);
                }
                continue;
            }

            if let Some(rest) = line.strip_prefix("model ") {
                let name = rest.split_whitespace().next().unwrap_or("");
                if !name.is_empty() {
                    on_model(name.trim_end_matches('{').trim());
                    if line.contains('{') {
                        active_block = Some(BlockKind::Model);
                    } else {
                        pending_block = Some(BlockKind::Model);
                    }
                }
                continue;
            }

            if let Some(rest) = line.strip_prefix("enum ") {
                let name = rest.split_whitespace().next().unwrap_or("");
                if !name.is_empty() {
                    if line.contains('{') {
                        active_block = Some(BlockKind::Enum);
                    } else {
                        pending_block = Some(BlockKind::Enum);
                    }
                }
            }
        }
    }

    /// Parses all .prisma files matching the schema glob and returns the set of model names.
    pub fn extract_model_names(schema_glob: &glob::Pattern, root: &Path) -> HashSet<String> {
        let mut model_names = HashSet::new();

        for file_path in Self::matching_schema_files(schema_glob, root) {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                Self::parse_schema_file(
                    &content,
                    |name| {
                        model_names.insert(name.to_string());
                    },
                    |_| {},
                    |_| {},
                );
            }
        }

        model_names
    }

    /// Parses all .prisma files matching the schema glob and returns the set of field names across all models.
    pub fn extract_field_names(schema_glob: &glob::Pattern, root: &Path) -> HashSet<String> {
        let mut field_names = HashSet::new();

        for file_path in Self::matching_schema_files(schema_glob, root) {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                Self::parse_schema_file(
                    &content,
                    |_| {},
                    |field| {
                        field_names.insert(field.to_string());
                    },
                    |_| {},
                );
            }
        }

        field_names
    }

    /// Parses all .prisma files matching the schema glob and returns the set of enum values across all enums.
    pub fn extract_enum_values(schema_glob: &glob::Pattern, root: &Path) -> HashSet<String> {
        let mut enum_values = HashSet::new();

        for file_path in Self::matching_schema_files(schema_glob, root) {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                Self::parse_schema_file(
                    &content,
                    |_| {},
                    |_| {},
                    |value| {
                        enum_values.insert(value.to_string());
                    },
                );
            }
        }

        enum_values
    }
}
