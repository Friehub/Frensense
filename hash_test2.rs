use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let mut h = FxHasher::default();
    let labels = vec!["UserInputSource", "taint_flow", "DbQuerySink"];
    labels.hash(&mut h);
    println!("UserInputSource -> DbQuerySink: {}", h.finish());
    
    let mut h2 = FxHasher::default();
    let labels2 = vec!["UserInputSource", "taint_flow", "HttpResponseSink"];
    labels2.hash(&mut h2);
    println!("UserInputSource -> HttpResponseSink: {}", h2.finish());

    let mut h3 = FxHasher::default();
    let labels3 = vec!["UserInputSource", "taint_flow", "PasswordHashing"];
    labels3.hash(&mut h3);
    println!("UserInputSource -> PasswordHashing: {}", h3.finish());
}
