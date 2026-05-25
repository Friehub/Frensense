fn fallible() -> Result<i32, String> {
    let x = do_something()?;
    Ok(x)
}
