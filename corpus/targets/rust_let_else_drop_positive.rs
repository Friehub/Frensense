// [frensense]
// observation: A `let ... else` pattern is used with a temporary that holds a mutex lock. The temporary is dropped at the end of the `else` block before the diverging branch, releasing the lock and causing a TOCTOU race between the check and the subsequent use.
// impact: The lock is released before the guarded operation completes, allowing another thread to mutate the shared state between the guard check and the actual access.
// improvement: Keep the guard alive in the same scope as the access, or use a single atomic operation instead of lock-and-check.

use std::sync::Mutex;

struct Cache {
    entries: Mutex<Vec<String>>,
}

fn first_entry(cache: &Cache) -> Option<String> {
    let entry = cache.entries.lock().ok()?.first()?;
    let result = entry.clone();
    Some(result)
}

fn main() {
    let cache = Cache { entries: Mutex::new(vec!["a".into(), "b".into()]) };
    println!("{:?}", first_entry(&cache));
}
