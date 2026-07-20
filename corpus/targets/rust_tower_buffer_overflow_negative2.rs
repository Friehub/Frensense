use tower::buffer::Buffer;
use tower::Service;
use std::num::NonZeroUsize;

pub fn create_pipeline<S>(svc: S) -> Buffer<S, S::Request>
where
    S: Service<S::Request> + Send + 'static,
    S::Future: Send,
{
    // SAFE: Bounded to 256 items with NonZeroUsize for correctness.
    let cap = NonZeroUsize::new(256).unwrap();
    Buffer::new(svc, cap.get())
}
