// [frensense]
// observation: A declarative macro uses `$tt:tt` to capture arbitrary token trees and paste them into the expansion without any transformation or validation, so the caller can inject items, statements, or expressions that break hygiene assumptions.
// impact: The macro can produce code with unexpected side effects, name collisions, or compile errors that are hard to debug. In downstream crates this can lead to code execution or logic bugs.
// improvement: Avoid bare `$tt:tt` captures; use more specific matchers (`$expr:expr`, `$ident:ident`, `$ty:ty`) and validate the input structure.

#[macro_export]
macro_rules! wrap_in_mod {
    ($name:ident, $body:tt) => {
        mod $name {
            $body
        }
    };
}

#[macro_export]
macro_rules! derive_debug_fallback {
    ($ty:ty, $tt:tt) => {
        impl ::core::fmt::Debug for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", stringify!($tt))
            }
        }
    };
}
