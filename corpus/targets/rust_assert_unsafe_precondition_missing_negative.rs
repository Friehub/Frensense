use core::hint::assert_unsafe_precondition;

fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    // SAFE: Precondition is checked in debug builds via assert_unsafe_precondition!
    unsafe {
        assert_unsafe_precondition!(
            "split_at_mut requires mid <= len",
            (mid: usize = mid, len: usize = len) => mid <= len,
        );
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
