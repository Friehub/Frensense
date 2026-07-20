use std::mem::offset_of;

#[repr(C)]
struct Header {
    tag: u8,
    kind: u64,
    data: [u8; 32],
}

fn write_tag(ptr: *mut Header, value: u8) {
    let offset = offset_of!(Header, tag);
    // SAFE: Header is #[repr(C)] so offset_of! yields a stable, guaranteed offset.
    unsafe { ptr.cast::<u8>().add(offset).write(value) }
}

fn main() {
    let mut h = Header { tag: 0, kind: 1, data: [0u8; 32] };
    write_tag(&mut h as *mut Header, 42);
    println!("{}", h.tag);
}
