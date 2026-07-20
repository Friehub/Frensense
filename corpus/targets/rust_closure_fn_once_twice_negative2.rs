// SAFE: Uses FnMut instead of FnOnce since the closure needs to be called multiple times.

fn run_multiple<F>(f: &mut F)
where
    F: FnMut(),
{
    f();
    f();
}
