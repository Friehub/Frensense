// SAFE: Specific matchers are used instead of bare `$tt:tt`.
#[macro_export]
macro_rules! wrap_in_mod {
    ($name:ident, $item:item) => {
        mod $name {
            $item
        }
    };
}

#[macro_export]
macro_rules! derive_debug_fallback {
    ($ty:ty, $expr:expr) => {
        impl ::core::fmt::Debug for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", $expr)
            }
        }
    };
}
