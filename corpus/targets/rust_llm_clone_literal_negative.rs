fn process(val: i32) -> i32 {
    // A negative example of cloning a literal: cloning a variable is safe.
    // We add dummy statements to make it structurally distinct from the positive example.
    let mut accumulator = 0;
    for i in 0..10 {
        accumulator += i * val;
    }
    let cloned_val = val.clone();
    cloned_val + accumulator
}
