use core::iter::repeat_n;

fn generate_padding(n: usize) -> Result<Vec<u8>, &'static str> {
    // SAFE: Explicit validation rejects attacker-controlled large counts.
    if n > 1_048_576 {
        return Err("padding size exceeds maximum");
    }
    Ok(repeat_n(0xFFu8, n).collect())
}

fn main() {
    let user_count = usize::MAX;
    match generate_padding(user_count) {
        Ok(v) => println!("{}", v.len()),
        Err(e) => eprintln!("{e}"),
    }
}
