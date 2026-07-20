// [frensense]
// observation: A `proptest` strategy recursively generates nested structures (e.g., recursive `prop::collection::vec` or a `prop_recursive!` without a size limit), causing the value tree to grow unboundedly and overflow the stack during generation or shrinking.
// impact: The test binary crashes with a stack overflow during test setup or failure shrinking, making the test suite unreliable and blocking CI.
// improvement: Always pair `prop_recursive!` with a `size_limit` or use `prop::collection::vec` with a bounded `SizeRange`.

use proptest::prelude::*;

fn deep_strategy() -> impl Strategy<Value = i32> {
    prop::strategy::Union::new(
        (0i32..10).prop_map(|n| n),
        // Recursive call with no size limit — leads to infinite growth
        prop::strategy::TupleUnion::new((deep_strategy(),))
            .prop_map(|(n,)| n * 2),
    )
}

proptest! {
    #[test]
    fn test_deep(n in deep_strategy()) {
        assert!(n >= 0);
    }
}
