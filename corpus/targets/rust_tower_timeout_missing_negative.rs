use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tower::{Layer, Service, timeout::TimeoutLayer};

pub fn timeout_stack<S>(inner: S) -> tower::timeout::Timeout<S> {
    // SAFE: TimeoutLayer bounds each request to 30 seconds max.
    TimeoutLayer::new(Duration::from_secs(30)).layer(inner)
}
