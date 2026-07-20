use std::sync::Mutex;

struct Cache {
    entries: Mutex<Vec<String>>,
}

fn first_entry(cache: &Cache) -> Option<String> {
    // SAFE: Guard is held across the entire access; no TOCTOU gap.
    let guard = cache.entries.lock().ok()?;
    guard.first().cloned()
}

fn main() {
    let cache = Cache { entries: Mutex::new(vec!["a".into(), "b".into()]) };
    println!("{:?}", first_entry(&cache));
}
