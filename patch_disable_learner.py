import re
content = open("frensense-bundler/src/pattern/weight_learner.rs").read()
content = content.replace("pub fn learn_category_weights(patterns: &[CorpusPattern]) -> HashMap<String, FeatureVec> {", "pub fn learn_category_weights(patterns: &[CorpusPattern]) -> HashMap<String, FeatureVec> {\n    let mut result = HashMap::new();\n    result.insert(\"_global\".to_string(), DEFAULT_WEIGHTS);\n    for p in patterns {\n        let cat = extract_category(&p.id).to_string();\n        result.insert(cat, DEFAULT_WEIGHTS);\n        result.insert(p.id.clone(), DEFAULT_WEIGHTS);\n    }\n    return result;\n")
open("frensense-bundler/src/pattern/weight_learner.rs", "w").write(content)
