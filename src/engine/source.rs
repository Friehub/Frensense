// SPDX-License-Identifier: MIT

use crate::FileId;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub id: FileId,
    pub path: PathBuf,
    pub content: Arc<str>,
}

#[derive(Default)]
pub struct SourceRegistry {
    files: HashMap<FileId, Arc<SourceFile>>,
    path_to_id: HashMap<PathBuf, FileId>,
    next_id: u32,
}

impl SourceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, path: &Path, content: String) -> FileId {
        if let Some(id) = self.path_to_id.get(path) {
            return *id;
        }

        let id = FileId(self.next_id);
        self.next_id += 1;

        let source_file = Arc::new(SourceFile {
            id,
            path: path.to_path_buf(),
            content: Arc::from(content),
        });

        self.files.insert(id, source_file);
        self.path_to_id.insert(path.to_path_buf(), id);
        id
    }

    pub fn get(&self, id: FileId) -> Option<Arc<SourceFile>> {
        self.files.get(&id).cloned()
    }

    pub fn get_by_path(&self, path: &Path) -> Option<Arc<SourceFile>> {
        let id = self.path_to_id.get(path)?;
        self.get(*id)
    }

    pub fn resolve_snippet(&self, id: FileId, start: u32, end: u32) -> Option<String> {
        let file = self.get(id)?;
        let start = start as usize;
        let end = end as usize;
        if end <= file.content.len() {
            Some(file.content[start..end].to_string())
        } else {
            None
        }
    }
}
