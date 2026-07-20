fn round_up_to_page(size: u64, page_size: u64) -> Option<u64> {
    // SAFE: checked_next_multiple_of returns None for zero or overflow instead of panicking.
    size.checked_next_multiple_of(page_size)
}

fn main() {
    let user_alignment = 0u64;
    match round_up_to_page(100, user_alignment) {
        Some(v) => println!("{v}"),
        None => eprintln!("invalid alignment"),
    }
}
