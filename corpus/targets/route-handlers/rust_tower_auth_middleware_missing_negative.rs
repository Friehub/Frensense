use tower::Service;
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;

pub struct AuthService<S> {
    inner: S,
    valid_token: String,
}

impl<S, Request> Service<Request> for AuthService<S>
where
    Request: headers::HeaderMapExt,
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|e| Box::new(e) as _)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        // SAFE: Token is validated before forwarding the request to the inner service.
        if let Some(val) = auth_header {
            if val == self.valid_token {
                return Box::pin(self.inner.call(req).map_err(|e| Box::new(e) as _));
            }
        }
        Box::pin(async { Err("unauthorized".into()) })
    }
}
