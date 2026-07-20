// [frensense]
// observation: A macro expansion generates an identifier that is not hygienic (e.g., uses `concat_idents!` or string concatenation for names), causing it to collide with user-defined identifiers or other macro invocations.
// impact: Name collisions can cause silent shadowing, incorrect dispatch, or compile errors when the same macro is used twice in the same scope, leading to hard-to-find bugs.
// improvement: Use `paste::paste!` with hygienic identifiers or pass names as explicit arguments rather than generating them from string concatenation.

#[macro_export]
macro_rules! make_getter {
    ($field:ident) => {
        fn get_$field(&self) -> &str {
            &self.$field
        }
    };
}

#[macro_export]
macro_rules! define_enum {
    ($name:ident, $($variant:ident),+) => {
        enum $name {
            $($variant,)+
        }
        impl $name {
            fn count() -> usize {
                $crate::count!($($variant),+)
            }
        }
    };
}

#[macro_export]
macro_rules! count {
    ($($x:ident),+) => { <[()]>::len(&[$(stringify!($x)),+]) };
}
