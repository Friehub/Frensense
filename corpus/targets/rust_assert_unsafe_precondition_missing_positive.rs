// [frensense]
// observation: An `unsafe` function performs unchecked pointer arithmetic or dereferencing without using `assert_unsafe_precondition!` to validate preconditions in debug builds. Callers can silently violate safety contracts.
// impact: Undefined behavior when preconditions are violated — no diagnostic in debug builds because the assertion is missing. Bugs that corrupt memory silently until a hard-to-debug crash in release.
// improvement: Use `core::hint::assert_unsafe_precondition!` to check preconditions in debug builds, or make the function safe if preconditions can be checked at compile time.

fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            core::slice::from_raw_parts_mut(ptr, mid),
            core::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut buf = vec![1, 2, 3];
    let (a, b) = split_at_mut(&mut buf, 5);
    println!("{:?} {:?}", a, b);
}
