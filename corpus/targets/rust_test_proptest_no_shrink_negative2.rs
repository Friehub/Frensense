// SAFE: Custom strategy uses `prop_compose!` which maintains shrinking.
use proptest::prelude::*;

prop_compose! {
    fn positive_int()(x in 0i32..1000) -> i32 { x }
}

proptest! {
    #[test]
    fn test_positive(x in positive_int()) {
        assert!(x > 0);
    }
}
