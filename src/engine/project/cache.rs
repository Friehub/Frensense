// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::path::Path;

/// Content-hash cache stored at `<project_root>/.frensense/cache.json`.
///
/// Maps file paths to blake3 hex hashes of their content. On subsequent runs,
/// files whose hash matches the cache are skipped entirely — no parse, no audit.
///
/// Safe because: unchanged content → same findings as last run.
/// The cache is invalidated when the engine version changes (version field) or
/// when the active language filter differs from the one used to build the cache.
#[derive(Default)]
pub struct FileCache {
    pub files: HashMap<String, String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CacheFile {
    version: u32,
    language_filter: Option<Vec<String>>,
    files: HashMap<String, String>,
}

impl FileCache {
    const CURRENT_VERSION: u32 = 2;

    /// Load cache from `.frensense/cache.json` under the project root.
    /// Returns an empty cache if the file doesn't exist, is corrupt, or
    /// was built under a different language filter.
    #[must_use]
    pub fn load(root: &Path, language_filter: Option<&[&str]>) -> Self {
        let cache_path = root.join(".frensense").join("cache.json");
        let cached = std::fs::read_to_string(&cache_path)
            .ok()
            .and_then(|s| serde_json::from_str::<CacheFile>(&s).ok())
            .filter(|c| c.version == Self::CURRENT_VERSION);

        let Some(cached) = cached else {
            return Self {
                files: HashMap::new(),
            };
        };

        // Invalidate if the language filter changed
        let current_filter: Option<Vec<String>> =
            language_filter.map(|f| f.iter().map(ToString::to_string).collect());
        if cached.language_filter != current_filter {
            return Self {
                files: HashMap::new(),
            };
        }

        Self {
            files: cached.files,
        }
    }

    /// Save cache to `.frensense/cache.json` under the project root.
    /// Creates the `.frensense/` directory if it doesn't exist.
    pub fn save(&self, root: &Path, language_filter: Option<&[&str]>) {
        if self.files.is_empty() {
            return;
        }
        let dir = root.join(".frensense");
        let cache_path = dir.join("cache.json");
        let wrapper = CacheFile {
            version: Self::CURRENT_VERSION,
            language_filter: language_filter.map(|f| f.iter().map(ToString::to_string).collect()),
            files: self.files.clone(),
        };
        if let Ok(content) = serde_json::to_string_pretty(&wrapper) {
            let _ = std::fs::create_dir_all(&dir);
            let _ = std::fs::write(&cache_path, content);
        }
    }

    /// Returns `true` if the file's content matches the cached hash.
    #[must_use]
    pub fn is_unchanged(&self, path: &Path, content: &str) -> bool {
        let hash = blake3::hash(content.as_bytes()).to_hex().to_string();
        self.files.get(&path_to_key(path)) == Some(&hash)
    }

    /// Update the cache entry for a file with its current content hash.
    pub fn update(&mut self, path: &Path, content: &str) {
        let hash = blake3::hash(content.as_bytes()).to_hex().to_string();
        self.files.insert(path_to_key(path), hash);
    }

    /// Remove a file from the cache (e.g., when a read error occurs).
    pub fn remove(&mut self, path: &Path) {
        self.files.remove(&path_to_key(path));
    }
}

fn path_to_key(path: &Path) -> String {
    path.to_string_lossy().to_string()
}
