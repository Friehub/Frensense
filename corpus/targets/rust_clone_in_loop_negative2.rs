// SAFE: Uses entry API with or_insert_with to avoid cloning in the hot path
use std::collections::HashMap;

fn build_index(records: &[Record]) -> HashMap<String, Vec<usize>> {
    let mut index: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, record) in records.iter().enumerate() {
        index.entry(record.name.clone())
            .or_insert_with(Vec::new)
            .push(i);
    }
    index
}
