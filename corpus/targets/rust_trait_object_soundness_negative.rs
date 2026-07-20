// SAFE: Non-object-safe method is gated behind `where Self: Sized`.
pub trait Process {
    fn process(&self);
    fn from_raw(data: &[u8]) -> Self where Self: Sized;
}

pub fn run_dynamic(t: &dyn Process) {
    t.process();
}
