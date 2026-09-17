use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let mut h = FxHasher::default();
    let labels = vec!["UserInputSource", "taint_flow", "SqlInjectionSink"];
    labels.hash(&mut h);
    println!("SqlInjectionSink: {}", h.finish());
    
    let mut h2 = FxHasher::default();
    let labels2 = vec!["DatabaseSource", "taint_flow", "SqlInjectionSink"];
    labels2.hash(&mut h2);
    println!("DatabaseSource -> SqlInjectionSink: {}", h2.finish());
}
