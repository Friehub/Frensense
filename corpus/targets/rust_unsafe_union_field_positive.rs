// [frensense]
// observation: Accessing a field of a `union` without knowing which variant is currently active. Union field access is always unsafe and reads uninitialized or wrong-type data if the wrong field is accessed.
// impact: Reading wrong-type data leads to undefined behavior — type confusion, information leaks, or crashes.
// improvement: Use `enum` instead of `union`, or ensure the active variant is tracked with a discriminant tag.

union Value {
    integer: i32,
    float: f64,
}

pub unsafe fn read_union(u: Value) -> i32 {
    u.integer
}
