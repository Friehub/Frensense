pub fn process_data(data: String) {
    // 'db_query' is a known sink in many configs, 
    // or we can just ensure the rule flags it.
    db_query(data);
}

fn db_query(q: String) {
    println!("Executing: {}", q);
}
