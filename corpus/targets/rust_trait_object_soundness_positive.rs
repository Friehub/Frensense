// [frensense]
// observation: A trait that is not object-safe (e.g., contains generic methods, `Self: Sized`, or methods that return `Self`) is used as a trait object via `dyn Trait`. The compiler may accept this in some contexts (e.g., behind `Box<dyn Trait>` with a where-clause) but the vtable dispatch is unsound and can produce UB.
// impact: Calling a method through a `dyn Trait` on a non-object-safe trait can produce undefined behavior at runtime — the wrong function may be called, arguments may be misaligned, or memory corruption may occur.
// improvement: Ensure the trait is object-safe by removing non-object-safe methods, or mark them with `where Self: Sized` so they are excluded from vtable dispatch.

pub trait Process {
    fn process(&self);
    fn from_raw(data: &[u8]) -> Self;
}

pub fn run_dynamic(t: &dyn Process) {
    t.process();
}
