content = open("frensense-engine/src/pattern/similarity.rs").read()
content = content.replace("""pub fn containment(candidate: &[u64], target: &[u64]) -> f64 {
    if target.is_empty() {
        return 1.0;
    }
    if candidate.is_empty() {
        return 0.0;
    }""", """pub fn containment(candidate: &[u64], target: &[u64]) -> f64 {
    if candidate.is_empty() && target.is_empty() {
        return 0.0;
    }
    if target.is_empty() {
        return 0.0; // If target is empty, we cannot contain it. Return 0.0 so it doesn't hallucinate 1.0.
    }
    if candidate.is_empty() {
        return 0.0;
    }""")
open("frensense-engine/src/pattern/similarity.rs", "w").write(content)
