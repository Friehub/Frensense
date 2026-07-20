// [frensense]
// observation: `std::mem::offset_of!` is used on a `#[repr(Rust)]` struct whose field layout is unspecified. The computed offset may be incorrect, and using it for raw-pointer arithmetic causes undefined behavior.
// impact: Field offsets computed at compile time may not match actual layout due to reordering, padding, or niche optimizations. Writing via the computed offset corrupts adjacent fields or causes memory unsafety.
// improvement: Apply `#[repr(C)]` to guarantee field order, or use safe field access instead of offset arithmetic.

use std::mem::offset_of;

struct Header {
    tag: u8,
    kind: u64,
    data: [u8; 32],
}

fn write_tag(ptr: *mut Header, value: u8) {
    let offset = offset_of!(Header, tag);
    unsafe { ptr.cast::<u8>().add(offset).write(value) }
}

fn main() {
    let mut h = Header { tag: 0, kind: 1, data: [0u8; 32] };
    write_tag(&mut h as *mut Header, 42);
    println!("{}", h.tag);
}
