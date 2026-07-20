// SAFE: Uses Pin + ownership instead of Box::into_raw; ensures the allocation lives for the correct lifetime without manual free.

use std::pin::Pin;

fn safe_use_after_free() -> u32 {
    let b = Pin::new(Box::new(42u32));
    let leaked: Pin<&u32> = b.as_ref();
    *leaked
}

fn safe_double() {
    let s = "hello".to_string();
    let p: Pin<&mut String> = Pin::new(&mut s.clone());
    drop(s);
}
