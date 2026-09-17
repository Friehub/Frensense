use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn main() {
    let mut h = DefaultHasher::new();
    "UserInputSource".hash(&mut h);
    "taint_flow".hash(&mut h);
    "DbQuerySink".hash(&mut h);
    println!("Hash: {}", h.finish());
}
