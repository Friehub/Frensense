// SAFE: Uses `paste` crate for hygienic concatenation.
#[macro_export]
macro_rules! make_getter {
    ($field:ident) => {
        paste::paste! {
            fn [<get_ $field>](&self) -> &str {
                &self.$field
            }
        }
    };
}

#[macro_export]
macro_rules! define_enum {
    ($name:ident, $($variant:ident),+) => {
        enum $name {
            $($variant,)+
        }
    };
}
