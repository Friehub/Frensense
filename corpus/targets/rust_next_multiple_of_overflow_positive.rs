// [frensense]
// observation: `{integer}::next_multiple_of` is called with zero as the divisor, causing a panic. When the divisor is dynamically computed from user input, this becomes a denial-of-service vector.
// impact: A panic crashes the current task. If the divisor is attacker-controlled, an attacker can trivially trigger a panic on every request, taking the service down.
// improvement: Validate the divisor is non-zero before calling `next_multiple_of`, or use `checked_next_multiple_of` which returns `None` instead of panicking.

fn round_up_to_page(size: u64, page_size: u64) -> u64 {
    size.next_multiple_of(page_size)
}

fn allocate(size: u64, alignment: u64) -> u64 {
    let aligned = round_up_to_page(size, alignment);
    aligned
}

fn main() {
    let user_alignment = 0u64;
    println!("{}", allocate(100, user_alignment));
}
