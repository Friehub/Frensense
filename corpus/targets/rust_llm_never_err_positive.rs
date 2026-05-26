fn compute_result() -> Result<i32, String> {
    let x = compute_value();
    if x > 0 { return Ok(x); } else { println!("noop"); }
    // No Err path, no ?, no None
    loop {}
}
