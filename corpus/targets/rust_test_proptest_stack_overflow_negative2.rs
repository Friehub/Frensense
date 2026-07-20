// SAFE: Uses `prop::collection::vec` with an explicit size bound instead of recursion.
use proptest::prelude::*;

fn bounded_strategy() -> impl Strategy<Value = Vec<i32>> {
    prop::collection::vec(0i32..10, 0..=5)
}

proptest! {
    #[test]
    fn test_bounded(v in bounded_strategy()) {
        for n in &v {
            assert!(*n >= 0);
        }
    }
}
