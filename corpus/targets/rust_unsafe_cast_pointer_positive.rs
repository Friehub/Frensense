// [frensense]
// observation: Raw pointers are cast between unrelated types, violating strict aliasing rules and causing undefined behavior.
// impact: The compiler may optimize the code in ways that break the intended behavior because it assumes pointers to different types do not alias.
// improvement: Use proper conversions like bytemuck::cast or design the types to share the same representation, or use std::mem::transmute with size checks.

use std::mem::transmute;

fn cast_u32_to_f32(p: *const u32) -> f32 {
    unsafe { *(p as *const f32) }
}

fn reinterpret_slice(data: &[u8]) -> &[u32] {
    unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u32, data.len() / 4) }
}

fn transmute_ref<T, U>(val: &T) -> &U {
    unsafe { &*(val as *const T as *const U) }
}
