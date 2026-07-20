// SAFE: Uses Weak references to break the cycle; the closure only holds a Weak reference to avoid preventing deallocation.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    callback: Option<Box<dyn Fn()>>,
}

fn create_cycle_free() -> Rc<RefCell<Node>> {
    let node = Rc::new(RefCell::new(Node {
        value: 42,
        callback: None,
    }));
    let weak = Rc::downgrade(&node);
    let cb = move || {
        if let Some(n) = weak.upgrade() {
            n.borrow_mut().value += 1;
        }
    };
    node.borrow_mut().callback = Some(Box::new(cb));
    node
}
