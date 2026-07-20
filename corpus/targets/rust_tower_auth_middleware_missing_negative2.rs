use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct AuthLayer {
    token: String,
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthService {
            inner,
            token: self.token.clone(),
        }
    }
}

pub struct AuthService<S> {
    inner: S,
    token: String,
}

impl<S, Request> Service<Request> for AuthService<S>
where
    S: Service<Request>,
    Request: headers::HeaderMapExt,
{
    type Response = S::Response;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|e| Box::new(e) as _)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        // SAFE: Every request goes through auth check before inner service.
        match req.headers().get("Authorization") {
            Some(val) if val == self.token => {}
            _ => return Box::pin(async { Err("forbidden".into()) }),
        }
        Box::pin(self.inner.call(req).map_err(|e| Box::new(e) as _))
    }
}
