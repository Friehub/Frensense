use std::collections::HashMap;

fn build_index(records: &[Record]) -> HashMap<String, Vec<usize>> {
    let mut index: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, record) in records.iter().enumerate() {
        let key = record.name.clone();
        let entry = index.entry(key).or_insert_with(Vec::new);
        entry.push(i);
    }
    index
}
