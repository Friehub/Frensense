fn clone_in_loop_case() {
    let v = vec![1, 2, 3];
    for x in v {
        let y = x.clone();
    }
}
