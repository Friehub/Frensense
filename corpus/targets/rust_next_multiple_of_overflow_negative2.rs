fn round_up_to_page(size: u64, page_size: u64) -> u64 {
    // SAFE: Explicit guard against zero ensures no panic.
    if page_size == 0 {
        return size;
    }
    size.next_multiple_of(page_size)
}

fn main() {
    let user_alignment = 0u64;
    println!("{}", round_up_to_page(100, user_alignment));
}
