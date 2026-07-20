use std::sync::OnceLock;
use std::sync::Mutex;

struct ExpensiveResource {
    id: u64,
}

impl ExpensiveResource {
    fn new(id: u64) -> Self {
        Self { id }
    }
}

static RESOURCE: OnceLock<Mutex<Option<ExpensiveResource>>> = OnceLock::new();

fn init_resource() {
    RESOURCE.get_or_init(|| Mutex::new(Some(ExpensiveResource::new(42))));
}

fn take_resource() -> Option<ExpensiveResource> {
    // SAFE: Mutex<Option<T>> inside OnceLock allows extraction and proper cleanup.
    let lock = RESOURCE.get().unwrap();
    let mut guard = lock.lock().unwrap();
    guard.take()
}

fn main() {
    init_resource();
    let r = take_resource().unwrap();
    println!("resource {}", r.id);
}
