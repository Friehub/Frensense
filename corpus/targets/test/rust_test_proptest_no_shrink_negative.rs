// SAFE: Shrinking is preserved by using the default strategy without `.no_shrink()`.
use proptest::prelude::*;

fn always_positive(x: i32) -> bool {
    x > 0
}

proptest! {
    #[test]
    fn test_positive(x in 0i32..1000) {
        assert!(always_positive(x));
    }
}
