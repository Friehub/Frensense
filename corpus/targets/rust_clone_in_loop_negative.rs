use std::collections::HashMap;

fn build_index(records: &[Record]) -> HashMap<String, Vec<usize>> {
    let mut index: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, record) in records.iter().enumerate() {
        index.entry(record.name.clone()).or_default().push(i);
    }
    index
}
