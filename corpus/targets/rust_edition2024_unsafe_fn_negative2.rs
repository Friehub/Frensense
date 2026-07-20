fn get_ref<T>(x: &T) -> &T {
    x
}

fn main() {
    let x = 42u8;
    let r = get_ref(&x);
    println!("{}", r);
}
