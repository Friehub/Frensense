// SAFE: `$tt:tt` is validated by a local helper macro that enforces structure before expansion.
#[macro_export]
macro_rules! validate_tt {
    ($name:ident, $body:tt) => {
        const _: () = {
            fn must_be_item() { $body; }
        };
    };
}

#[macro_export]
macro_rules! wrap_in_mod {
    ($name:ident, $body:tt) => {
        validate_tt!($name, $body);
        mod $name {
            $body
        }
    };
}
