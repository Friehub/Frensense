use std::sync::Mutex;

struct Cache {
    entries: Mutex<Vec<String>>,
}

fn first_entry(cache: &Cache) -> Option<String> {
    // SAFE: Using std::mem::ManuallyDrop or extending the scope prevents premature lock release.
    let guard = cache.entries.lock().ok()?;
    let result = guard.first().cloned();
    drop(guard);
    result
}

fn main() {
    let cache = Cache { entries: Mutex::new(vec!["a".into(), "b".into()]) };
    println!("{:?}", first_entry(&cache));
}
