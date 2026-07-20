// SAFE: Union access is guarded by an explicit discriminant that is always checked.
union Value {
    integer: i32,
    float: f64,
}

struct TaggedValue {
    tag: u8,
    value: Value,
}

pub fn read_integer(tv: &TaggedValue) -> Option<i32> {
    if tv.tag == 0 {
        Some(unsafe { tv.value.integer })
    } else {
        None
    }
}
