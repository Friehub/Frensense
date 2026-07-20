// [frensense]
// observation: `Pin::new()` is called on a value of a type that implements `Unpin` (e.g., most standard library types), which makes the `Pin` a no-op — the value can still be moved freely. The developer likely intended to create a pinned, immovable value but the type is automatically `Unpin`.
// impact: The intended pinning guarantee is silently broken. If the pinned value is later moved (e.g., swapped out of a `Pin<&mut T>` via `mem::replace`), self-referential pointers within the struct become dangling, causing use-after-free or undefined behavior.
// improvement: Use `std::pin::Pin` on a type that is `!Unpin`, typically by adding a `PhantomPinned` field to the struct.

use std::pin::Pin;

pub struct SelfReferential {
    data: String,
    ptr: *const String,
}

impl SelfReferential {
    pub fn new(data: String) -> Self {
        let mut s = Self { ptr: std::ptr::null(), data };
        s.ptr = &s.data;
        s
    }
}

pub fn example() {
    let mut val = SelfReferential::new("hello".into());
    let pinned = Pin::new(&mut val);
    // pinned is a no-op because SelfReferential is Unpin!
}
