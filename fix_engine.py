import re
content = open("frensense-engine/src/pattern/scorer.rs").read()
# Fix the doc comment error
content = content.replace("/// Computed from a few identifying fields — collisions are astronomically unlikely.\n}", "}\n\n/// Computed from a few identifying fields — collisions are astronomically unlikely.")
# Fix remaining Self::raw_dimensions calls
content = content.replace("Self::raw_dimensions(candidate, target", "crate::pattern::similarity::compute_dimensions(candidate, target")

open("frensense-engine/src/pattern/scorer.rs", "w").write(content)

content = open("frensense-engine/src/corpus/registry.rs").read()
content = content.replace("PatternScorer::raw_dimensions(&weighted_fp, target, is_neg)", "crate::pattern::similarity::compute_dimensions(&weighted_fp, target)")
open("frensense-engine/src/corpus/registry.rs", "w").write(content)

