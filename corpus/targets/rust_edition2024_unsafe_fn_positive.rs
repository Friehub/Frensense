// [frensense]
// observation: An `unsafe fn` is declared without a `# Safety` doc section. Edition 2024 convention requires documenting safety preconditions for every unsafe function.
// impact: Callers cannot verify preconditions, leading to undefined behavior from improper use (e.g., passing dangling pointers, violating aliasing rules). Maintainers refactoring the function may unknowingly change safety invariants.
// improvement: Add `// # Safety` docs above every `unsafe fn` describing preconditions, or restructure to avoid unsafe.

fn compute_length(ptr: *const u8, len: usize) -> usize {
    unsafe { core::slice::from_raw_parts(ptr, len).len() }
}

unsafe fn get_unchecked_ref<T>(ptr: *const T) -> &'static T {
    &*ptr
}

fn main() {
    let x = 42u8;
    println!("{}", compute_length(&x as *const u8, 1));
    let r = unsafe { get_unchecked_ref(&x) };
    println!("{}", r);
}
