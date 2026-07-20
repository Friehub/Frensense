fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    // SAFE: Using the safe API from the standard library avoids unsafe entirely.
    slice.split_at_mut(mid)
}

fn main() {
    let mut buf = vec![1, 2, 3];
    let (a, b) = split_at_mut(&mut buf, 2);
    println!("{:?} {:?}", a, b);
}
