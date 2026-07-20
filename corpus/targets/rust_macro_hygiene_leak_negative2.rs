// SAFE: Macro explicitly takes generated names as parameters instead of constructing them.
#[macro_export]
macro_rules! make_getter {
    ($getter:ident, $field:ident) => {
        fn $getter(&self) -> &str {
            &self.$field
        }
    };
}
