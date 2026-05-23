fn test() {
    let x = String::from("hello");
    let y = x.clone();
    for _ in 0..3 {
        println!("{}", y);
    }
}
