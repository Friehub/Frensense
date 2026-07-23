// [frensense]
// observation: A Tower Service is constructed without any authentication middleware. Incoming requests are forwarded directly to the inner service without verifying the caller's identity.
// impact: Unauthenticated access to protected resources. Any remote client can invoke the service without credentials.
// improvement: Wrap the inner service with an auth-checking middleware that inspects headers, tokens, or certificates before delegating to the inner service.

use tower::Service;
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;

pub struct NoAuthService<S> {
    inner: S,
}

impl<S, Request> Service<Request> for NoAuthService<S>
where
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        self.inner.call(req)
    }
}
