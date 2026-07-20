// [frensense]
// observation: A `OnceCell` or `OnceLock` is used to store a non-clonable value that needs to be accessed by multiple consumers after initialization. Since `OnceLock::get()` returns `Option<&T>`, the caller must clone or take ownership — but if `T` is neither `Clone` nor `Default`, there is no way to extract the value once set without leaking it via raw pointer manipulation.
// impact: Value leak — the initialized value can never be extracted from the OnceCell, leading to resource leaks (e.g., database connections that should be closed, file handles, or memory-allocated structures). The value lives for the remainder of the program with no way to cleanly drop it.
// improvement: Ensure the stored type is `Clone` if multiple consumers need it, or use `Arc<T>` inside the OnceCell so the value can be shared by cloning the Arc.

use std::sync::OnceLock;

struct ExpensiveResource {
    id: u64,
}

impl ExpensiveResource {
    fn new(id: u64) -> Self {
        Self { id }
    }
}

static RESOURCE: OnceLock<ExpensiveResource> = OnceLock::new();

fn get_resource() -> &'static ExpensiveResource {
    RESOURCE.get_or_init(|| ExpensiveResource::new(42))
}

fn main() {
    let r = get_resource();
    println!("resource {}", r.id);
}
