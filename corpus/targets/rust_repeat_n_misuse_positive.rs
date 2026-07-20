// [frensense]
// observation: `core::iter::repeat_n` is called with a very large count (e.g., from user input) and the iterator is collected without bound. This can allocate huge amounts of memory or cause an integer overflow in the count.
// impact: Unbounded memory allocation can lead to OOM denial of service. If `n` is attacker-controlled, they can exhaust server memory.
// improvement: Use `.take()` with a sane maximum, validate the count before constructing the iterator, or use a bounded collect strategy.

use core::iter::repeat_n;

fn generate_padding(n: usize) -> Vec<u8> {
    repeat_n(0xFFu8, n).collect()
}

fn main() {
    let user_count = usize::MAX;
    let v = generate_padding(user_count);
    println!("{}", v.len());
}
