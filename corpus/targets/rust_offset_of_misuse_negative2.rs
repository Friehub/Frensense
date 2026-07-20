struct Header {
    tag: u8,
    kind: u64,
    data: [u8; 32],
}

fn set_tag(h: &mut Header, value: u8) {
    // SAFE: Using safe field access instead of raw offset arithmetic avoids UB entirely.
    h.tag = value;
}

fn main() {
    let mut h = Header { tag: 0, kind: 1, data: [0u8; 32] };
    set_tag(&mut h, 42);
    println!("{}", h.tag);
}
