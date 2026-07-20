// [frensense]
// observation: A sensitive route (e.g., login, password reset) is served through a Tower service stack that has no rate limiting middleware. Requests pass directly to the inner service without any throttling.
// impact: Brute-force attacks on authentication endpoints are unmitigated. Attackers can send unlimited password guesses or token requests without slowing down.
// improvement: Add a rate-limiting layer (e.g., `tower::limit` or a token-bucket middleware) that restricts requests per client IP or per user.

use tower::Service;
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;

pub struct LoginService;

impl Service<String> for LoginService {
    type Response = String;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: String) -> Self::Future {
        Box::pin(async move { Ok(format!("logged in as {}", req)) })
    }
}
