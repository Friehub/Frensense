import re

with open('frensense-bundler/src/builder.rs', 'r') as f:
    text = f.read()

replacement = """
    fn collect_corpus_files(dir: &Path, map: &mut std::collections::HashMap<String, String>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    collect_corpus_files(&path, map);
                } else if path.is_file() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            map.insert(name.to_string(), content);
                        }
                    }
                }
            }
        }
    }

    let mut corpus_files = std::collections::HashMap::new();
    collect_corpus_files(corpus_dir.as_path(), &mut corpus_files);

    fn find_corpus_file(id: &str, variant: &str, corpus_files: &std::collections::HashMap<String, String>) -> Option<String> {
        for ext in &["ts", "tsx", "js", "jsx", "rs", "py", "go"] {
            let target = format!("{}_{}.{}", id, variant, ext);
            if let Some(content) = corpus_files.get(&target) {
                return Some(content.clone());
            }
        }
        None
    }

    let mut pattern_source_texts = std::collections::HashMap::new();
    for bp in patterns {
        if let Some(src) = find_corpus_file(&bp.id, "positive", &corpus_files) {
            pattern_source_texts.insert(bp.id.clone(), src);
        }
        for (i, variant) in ["negative", "negative2", "negative3", "negative4"].iter().enumerate() {
            if let Some(src) = find_corpus_file(&bp.id, variant, &corpus_files) {
                pattern_source_texts.insert(format!("{}_neg_{}", bp.id, i), src);
            }
        }
    }
"""

start_pattern = r"    fn find_corpus_file\(id: &str, variant: &str, dir: &Path\) -> Option<String> \{"
end_pattern = r"                pattern_source_texts\.insert\(format!\(\"\{\}_neg_\{\}\", bp\.id, i\), src\);\n            \}\n        \}\n    \}"

# Find the block and replace it
s = re.search(start_pattern, text)
e = re.search(end_pattern, text)

if s and e:
    new_text = text[:s.start()] + replacement + text[e.end():]
    with open('frensense-bundler/src/builder.rs', 'w') as f:
        f.write(new_text)
    print("Patched successfully")
else:
    print("Could not find patterns")
