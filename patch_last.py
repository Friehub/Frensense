content = open("frensense-engine/src/pattern/scorer.rs").read()
content = content.replace("crate::pattern::similarity::compute_dimensions(candidate, target, !_is_positive)", "crate::pattern::similarity::compute_dimensions(candidate, target)")
content = content.replace("/// A lightweight identity-hash for a fingerprint, used as a cache key.\n/// Computed from a few identifying fields — collisions are astronomically unlikely.", "// A lightweight identity-hash for a fingerprint, used as a cache key.\n// Computed from a few identifying fields — collisions are astronomically unlikely.")
open("frensense-engine/src/pattern/scorer.rs", "w").write(content)
