// SAFE: Use `prop_recursive!` with a depth limit to bound recursion.
use proptest::prelude::*;

fn deep_strategy() -> impl Strategy<Value = i32> {
    prop::strategy::Union::new(
        (0i32..10).prop_map(|n| n),
        // Recursive with depth limit
        prop::strategy::TupleUnion::new((deep_strategy(),))
            .prop_map(|(n,)| n * 2)
            .boxed(),
    )
    .prop_recursive(1, 4, 10, |inner| inner)
}

proptest! {
    #[test]
    fn test_deep(n in deep_strategy()) {
        assert!(n >= 0);
    }
}
