// [frensense]
// observation: An `unsafe trait` is defined with a default method implementation that performs an unsafe operation without requiring the implementor to verify the safety invariant. A downstream implementor may use the default and unknowingly violate safety.
// impact: The default method may perform pointer dereferences, transmutes, or other unsafe operations that assume the implementor satisfies invariants they never explicitly verified. This leads to undefined behavior at runtime when the default method is called.
// improvement: Do not provide default implementations for unsafe traits — force each implementor to write the implementation explicitly, or gate defaults behind a safe wrapper.

pub unsafe trait Zeroable {
    fn zeroed() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[derive(Clone)]
pub struct Handle {
    ptr: *mut u8,
}

unsafe impl Zeroable for Handle {}
