use tower::buffer::Buffer;
use tower::Service;

pub fn create_pipeline<S>(svc: S) -> Buffer<S, S::Request>
where
    S: Service<S::Request> + Send + 'static,
    S::Future: Send,
{
    // SAFE: Bounded to 1024 items; back-pressure propagates via poll_ready.
    Buffer::new(svc, 1024)
}
