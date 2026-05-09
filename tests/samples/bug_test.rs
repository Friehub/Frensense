fn main() {
    let v = vec![1, 2, 3];
    // gensense-ignore: RUST_CLONE_IN_LOOP
    for x in v {
        let y = x.clone();
        println!("{}", y);
    }
}
