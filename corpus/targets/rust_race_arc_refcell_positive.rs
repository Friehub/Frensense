// [frensense]
// observation: Arc<RefCell<T>> is used to share mutable state across threads. RefCell is not Sync, so this will fail to compile or exhibit undefined behavior when used incorrectly across threads.
// impact: If the code compiles (e.g., using unsafe or a wrapper), concurrent access via RefCell can cause runtime borrow panics or data races because RefCell has no thread synchronization.
// improvement: Use Arc<Mutex<T>> or Arc<RwLock<T>> instead of Arc<RefCell<T>> for cross-thread shared mutable state.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
struct SharedState {
    data: Arc<RefCell<Vec<String>>>,
}

fn add_item(state: &SharedState, item: String) {
    state.data.borrow_mut().push(item);
}

fn get_items(state: &SharedState) -> Vec<String> {
    state.data.borrow().clone()
}

fn process(state: SharedState) {
    add_item(&state, "task1".into());
    let items = get_items(&state);
    println!("Items: {:?}", items);
}
