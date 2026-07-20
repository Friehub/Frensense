use core::iter::repeat_n;

const MAX_PADDING: usize = 1_048_576;

fn generate_padding(n: usize) -> Vec<u8> {
    // SAFE: Bounding the count prevents OOM from unbounded allocation.
    let clamped = n.min(MAX_PADDING);
    repeat_n(0xFFu8, clamped).collect()
}

fn main() {
    let user_count = usize::MAX;
    let v = generate_padding(user_count);
    println!("{}", v.len());
}
