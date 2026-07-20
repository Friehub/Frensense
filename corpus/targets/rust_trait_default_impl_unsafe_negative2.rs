// SAFE: Default is provided via a safe wrapper trait, not the unsafe trait.
pub unsafe trait Zeroable {
    fn zeroed() -> Self;
}

pub trait SafeZeroable: Zeroable + Sized {
    fn safe_zeroed() -> Self {
        unsafe { Self::zeroed() }
    }
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

impl SafeZeroable for Handle {}
