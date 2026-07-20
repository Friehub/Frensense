// [frensense]
// observation: A closure captures Rc<RefCell<...>>, creating a reference cycle when the closure itself is stored inside the same Rc, causing a memory leak.
// impact: The reference cycle prevents the Rc reference count from reaching zero, causing a memory leak that grows with each cycle of operation.
// improvement: Use Weak references to break cycles, or restructure ownership to avoid cycles with Rc.

use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i32,
    callback: Option<Box<dyn Fn()>>,
}

fn create_cycle() -> Rc<RefCell<Node>> {
    let node = Rc::new(RefCell::new(Node {
        value: 42,
        callback: None,
    }));
    let weak = Rc::downgrade(&node);
    let cb = {
        let node_clone = node.clone();
        move || {
            if let Some(n) = weak.upgrade() {
                n.borrow_mut().value += 1;
            }
        }
    };
    node.borrow_mut().callback = Some(Box::new(cb));
    node
}

fn build_self_ref() -> Rc<RefCell<Vec<Box<dyn Fn()>>>> {
    let items = Rc::new(RefCell::new(Vec::new()));
    let captured = items.clone();
    items.borrow_mut().push(Box::new(move || {
        captured.borrow().len();
    }));
    items
}
