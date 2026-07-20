// [frensense]
// observation: A `proptest` strategy is defined without a custom `Shrink` implementation or without using `prop::strategy::Strategy`'s default shrinking. When a test failure is found, the shrinker is disabled or not tuned, so the minimal failing case is not found.
// impact: Failure reports include large, hard-to-read counterexamples instead of minimal ones, making debugging significantly harder and slower, especially for complex strategy trees.
// improvement: Ensure the strategy supports shrinking (default for most combinators). Use `prop::collection` helpers or `prop_compose!` which preserve shrinking. Avoid `prop::strategy::FnStrategy` or `Just` wrapping that discards shrinking.

use proptest::prelude::*;

fn always_positive(x: i32) -> bool {
    x > 0
}

proptest! {
    #[test]
    fn test_positive(x in (0i32..1000).prop_map(|v| v).no_shrink()) {
        assert!(always_positive(x));
    }
}
