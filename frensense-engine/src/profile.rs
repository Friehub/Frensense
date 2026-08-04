// SPDX-License-Identifier: MIT

use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::path::Path;

use crate::fingerprint::FunctionFingerprint;

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct ProfileEntry {
    pub count: usize,
    pub examples: Vec<String>,
}

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct LanguageProfile {
    pub total_ngrams: usize,
    pub total_functions: usize,
    pub body_ngram_freq: FxHashMap<u64, ProfileEntry>,
    pub signature_ngram_freq: FxHashMap<u64, ProfileEntry>,
    pub param_type_freq: FxHashMap<u64, ProfileEntry>,
    pub name_segment_freq: HashMap<String, ProfileEntry>,
    pub structural_marker_freq: FxHashMap<u64, ProfileEntry>,
    pub type_usage_freq: HashMap<String, ProfileEntry>,
    pub file_profiles: HashMap<String, FileProfile>,
}

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct FileProfile {
    pub body_ngram_freq: FxHashMap<u64, usize>,
    pub total_ngrams: usize,
}

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ProjectProfile {
    pub version: u32,
    pub languages: HashMap<String, LanguageProfile>,
    pub threshold: f64,
}

impl Default for ProjectProfile {
    fn default() -> Self {
        Self {
            version: 1,
            languages: HashMap::new(),
            threshold: 0.7,
        }
    }
}

fn dir_prefix(path: &str) -> String {
    let p = Path::new(path);
    p.parent()
        .and_then(|parent| parent.to_str())
        .map(|s| {
            // Group at the module level, not the top-level namespace. Using the
            // first two segments (the old behavior) collapsed `src/modules/auth/`
            // and `src/modules/billing/` into the same bucket, defeating
            // per-directory profile surprise detection. Instead take the
            // deepest 2-3 segments of the parent directory, or the whole parent
            // when it is shallow (i.e. whichever is shorter).
            let parts: Vec<&str> = s.split('/').filter(|p| !p.is_empty()).collect();
            let take = parts.len().min(3);
            if take > 0 {
                parts[parts.len() - take..].join("/")
            } else {
                s.to_string()
            }
        })
        .unwrap_or_default()
}

#[derive(Debug)]
pub struct StyleSurpriseResult {
    pub score: f64,
    pub details: Vec<String>,
}

impl ProjectProfile {
    pub fn learn(fingerprints: &[FunctionFingerprint]) -> Self {
        let mut profile = ProjectProfile::default();

        for fp in fingerprints {
            let lang_profile = profile.languages.entry(fp.language.clone()).or_default();
            lang_profile.total_functions += 1;

            for &hash in &fp.ngram_hashes {
                let entry =
                    lang_profile
                        .body_ngram_freq
                        .entry(hash)
                        .or_insert_with(|| ProfileEntry {
                            count: 0,
                            examples: Vec::new(),
                        });
                entry.count += 1;
                if entry.examples.len() < 3 {
                    entry
                        .examples
                        .push(format!("{}:{}", fp.function_name, fp.line));
                }
            }
            lang_profile.total_ngrams += fp.ngram_hashes.len();

            for &hash in &fp.signature_ngrams {
                let entry = lang_profile
                    .signature_ngram_freq
                    .entry(hash)
                    .or_insert_with(|| ProfileEntry {
                        count: 0,
                        examples: Vec::new(),
                    });
                entry.count += 1;
                if entry.examples.len() < 3 {
                    entry
                        .examples
                        .push(format!("{}:{}", fp.function_name, fp.line));
                }
            }

            for &hash in &fp.param_type_ngrams {
                let entry =
                    lang_profile
                        .param_type_freq
                        .entry(hash)
                        .or_insert_with(|| ProfileEntry {
                            count: 0,
                            examples: Vec::new(),
                        });
                entry.count += 1;
                if entry.examples.len() < 3 {
                    entry
                        .examples
                        .push(format!("{}:{}", fp.function_name, fp.line));
                }
            }

            for seg in &fp.name_segments {
                let entry = lang_profile
                    .name_segment_freq
                    .entry(seg.clone())
                    .or_insert_with(|| ProfileEntry {
                        count: 0,
                        examples: Vec::new(),
                    });
                entry.count += 1;
                if entry.examples.len() < 3 {
                    entry
                        .examples
                        .push(format!("{}:{}", fp.function_name, fp.line));
                }
            }

            for &hash in &fp.structural_markers {
                let entry = lang_profile
                    .structural_marker_freq
                    .entry(hash)
                    .or_insert_with(|| ProfileEntry {
                        count: 0,
                        examples: Vec::new(),
                    });
                entry.count += 1;
                if entry.examples.len() < 3 {
                    entry
                        .examples
                        .push(format!("{}:{}", fp.function_name, fp.line));
                }
            }

            for ty in &fp.type_usages {
                let entry = lang_profile
                    .type_usage_freq
                    .entry(ty.clone())
                    .or_insert_with(|| ProfileEntry {
                        count: 0,
                        examples: Vec::new(),
                    });
                entry.count += 1;
                if entry.examples.len() < 3 {
                    entry
                        .examples
                        .push(format!("{}:{}", fp.function_name, fp.line));
                }
            }

            let dir = dir_prefix(&fp.file_path);
            let file_profile =
                lang_profile
                    .file_profiles
                    .entry(dir)
                    .or_insert_with(|| FileProfile {
                        body_ngram_freq: FxHashMap::default(),
                        total_ngrams: 0,
                    });
            for &hash in &fp.ngram_hashes {
                *file_profile.body_ngram_freq.entry(hash).or_insert(0) += 1;
            }
            file_profile.total_ngrams += fp.ngram_hashes.len();
        }

        profile
    }

    #[allow(clippy::too_many_lines)]
    pub fn style_surprise(&self, fp: &FunctionFingerprint) -> StyleSurpriseResult {
        let Some(lang_profile) = self.languages.get(&fp.language) else {
            return StyleSurpriseResult {
                score: 1.0,
                details: vec!["Language not present in profile".to_string()],
            };
        };

        let mut total_features = 0usize;
        let mut unseen_features = 0usize;
        let mut details = Vec::new();

        for &hash in &fp.ngram_hashes {
            total_features += 1;
            if !lang_profile.body_ngram_freq.contains_key(&hash) {
                unseen_features += 1;
            }
        }
        if !fp.ngram_hashes.is_empty() {
            let ratio = unseen_features as f64 / fp.ngram_hashes.len() as f64;
            if ratio > 0.5 {
                details.push(format!(
                    "Body patterns: {:.0}% unfamiliar ({} unseen / {} total)",
                    ratio * 100.0,
                    unseen_features,
                    fp.ngram_hashes.len()
                ));
            }
        }

        let mut sig_unseen = 0usize;
        for &hash in &fp.signature_ngrams {
            total_features += 1;
            if !lang_profile.signature_ngram_freq.contains_key(&hash) {
                unseen_features += 1;
                sig_unseen += 1;
            }
        }
        if !fp.signature_ngrams.is_empty() {
            let ratio = sig_unseen as f64 / fp.signature_ngrams.len() as f64;
            if ratio > 0.5 {
                details.push(format!(
                    "Signature patterns: {:.0}% unfamiliar",
                    ratio * 100.0
                ));
            }
        }

        let mut name_seg_unseen = 0usize;
        for seg in &fp.name_segments {
            total_features += 1;
            if !lang_profile.name_segment_freq.contains_key(seg) && !seg.is_empty() {
                unseen_features += 1;
                name_seg_unseen += 1;
            }
        }
        if name_seg_unseen > 0 {
            for seg in &fp.name_segments {
                if !lang_profile.name_segment_freq.contains_key(seg) && !seg.is_empty() {
                    if let Some(entry) = lang_profile
                        .name_segment_freq
                        .iter()
                        .find(|(k, _)| k.to_lowercase() == seg.to_lowercase())
                    {
                        details.push(format!(
                            "Name casing: '{}' uses '{}' — project uses '{}' (seen {}x).",
                            fp.function_name, seg, entry.0, entry.1.count
                        ));
                    } else {
                        details.push(format!(
                            "Name segment: '{}' uses '{}' — never seen in this project.",
                            fp.function_name, seg
                        ));
                    }
                }
            }
        }

        let mut struct_unseen = 0usize;
        for &hash in &fp.structural_markers {
            total_features += 1;
            if !lang_profile.structural_marker_freq.contains_key(&hash) {
                unseen_features += 1;
                struct_unseen += 1;
            }
        }
        if !fp.structural_markers.is_empty() {
            let ratio = struct_unseen as f64 / fp.structural_markers.len() as f64;
            if ratio > 0.5 {
                details.push(format!(
                    "Structural markers: {:.0}% unfamiliar",
                    ratio * 100.0
                ));
            }
        }

        let mut ty_unseen = 0usize;
        for ty in &fp.type_usages {
            total_features += 1;
            if !lang_profile.type_usage_freq.contains_key(ty) {
                unseen_features += 1;
                ty_unseen += 1;
            }
        }
        if ty_unseen > 0 {
            for ty in &fp.type_usages {
                if !lang_profile.type_usage_freq.contains_key(ty) {
                    if let Some(entry) = lang_profile.type_usage_freq.get(ty) {
                        details.push(format!(
                            "Type usage: '{}' — seen {}x in project.",
                            ty, entry.count
                        ));
                    } else {
                        details.push(format!("Type '{ty}' never used in this project."));
                    }
                }
            }
        }

        if lang_profile.total_functions > 5 {
            let avg_density = lang_profile.body_ngram_freq.len() as f64
                / lang_profile.total_functions.max(1) as f64;
            if fp.comment_density < 0.01 && avg_density > 0.05 {
                details.push(
                    "No comments in function body — project average suggests ~5% comment density."
                        .to_string(),
                );
            }
        }

        let score = if total_features > 0 {
            unseen_features as f64 / total_features as f64
        } else {
            0.0
        };

        StyleSurpriseResult { score, details }
    }

    pub fn file_profile_surprise(&self, fp: &FunctionFingerprint) -> f64 {
        let Some(lang_profile) = self.languages.get(&fp.language) else {
            return 1.0;
        };
        let dir = dir_prefix(&fp.file_path);
        let Some(file_profile) = lang_profile.file_profiles.get(&dir) else {
            return 0.0;
        };
        if file_profile.total_ngrams == 0 {
            return 0.0;
        }
        let mut unseen: usize = 0;
        for &hash in &fp.ngram_hashes {
            if !file_profile.body_ngram_freq.contains_key(&hash) {
                unseen += 1;
            }
        }
        if fp.ngram_hashes.is_empty() {
            return 0.0;
        }
        unseen as f64 / fp.ngram_hashes.len() as f64
    }

    #[cfg(feature = "serialize")]
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, json)
    }

    #[cfg(feature = "serialize")]
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        serde_json::from_str(&data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}
