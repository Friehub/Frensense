import re
content = open("frensense-engine/src/pattern/similarity.rs").read()
lcs = """pub fn lcs_similarity(candidate: &[u64], target: &[u64]) -> f64 {
    let n = candidate.len();
    let m = target.len();
    if n == 0 || m == 0 {
        return 0.0;
    }
    // Prevent OOM: if sequence is too large, fallback to jaccard
    if n > 2000 || m > 2000 {
        return jaccard_sorted(candidate, target);
    }
    // Use O(min(N,M)) space: only need two rows
    let mut prev = vec![0; m + 1];
    let mut curr = vec![0; m + 1];
    for i in 1..=n {
        for j in 1..=m {
            if candidate[i - 1] == target[j - 1] {
                curr[j] = prev[j - 1] + 1;
            } else {
                curr[j] = std::cmp::max(prev[j], curr[j - 1]);
            }
        }
        prev.copy_from_slice(&curr);
    }
    let lcs = curr[m] as f64;
    let max_len = std::cmp::max(n, m) as f64;
    lcs / max_len
}"""
content = re.sub(r'pub fn lcs_similarity.*?\}\n\}', lcs + '\n', content, flags=re.DOTALL)
open("frensense-engine/src/pattern/similarity.rs", "w").write(content)
