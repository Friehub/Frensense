// SAFE: Avoids cloning entirely by using the literal value directly
fn process() -> i32 {
    let x = 42;
    let mut result = x;
    for i in 0..10 {
        result += i;
    }
    result
}
