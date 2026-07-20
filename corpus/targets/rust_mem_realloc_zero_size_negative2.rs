// SAFE: Uses Vec instead of manual realloc, which handles zero-size allocations correctly without UB.

fn safe_vec_realloc() {
    let mut v: Vec<u32> = Vec::new();
    v.push(1);
    v.clear();
    // Vec::shrink_to_fit with zero elements is safe
    v.shrink_to_fit();
}
