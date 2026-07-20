use std::sync::Arc;
use std::sync::OnceLock;

#[derive(Clone)]
struct ExpensiveResource {
    id: u64,
}

impl ExpensiveResource {
    fn new(id: u64) -> Self {
        Self { id }
    }
}

static RESOURCE: OnceLock<Arc<ExpensiveResource>> = OnceLock::new();

fn get_resource() -> Arc<ExpensiveResource> {
    // SAFE: Arc provides shared ownership; OnceLock ensures single initialization, Arc enables cloning out the value.
    RESOURCE.get_or_init(|| Arc::new(ExpensiveResource::new(42))).clone()
}

fn main() {
    let r = get_resource();
    println!("resource {}", r.id);
}
