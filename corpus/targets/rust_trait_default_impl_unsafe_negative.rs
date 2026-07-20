// SAFE: No default method — each implementor must provide their own safe version.
pub unsafe trait Zeroable {
    fn zeroed() -> Self;
}

#[derive(Clone)]
pub struct Handle {
    ptr: *mut u8,
}

unsafe impl Zeroable for Handle {
    fn zeroed() -> Self {
        Self { ptr: std::ptr::null_mut() }
    }
}
