// SAFE: The trait is split into object-safe and non-object-safe parts.
pub trait Process {
    fn process(&self);
}

pub trait ProcessExt: Process {
    fn from_raw(data: &[u8]) -> Self where Self: Sized;
}

pub fn run_dynamic(t: &dyn Process) {
    t.process();
}
