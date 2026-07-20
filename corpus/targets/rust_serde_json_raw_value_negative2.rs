// SAFE: Deserializes into a strongly-typed struct with deny_unknown_fields.
use serde::Deserialize;
use warp::Filter;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HelloRequest {
    pub name: String,
}

pub async fn handler(body: HelloRequest) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("hello {}", body.name))
}

pub fn route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post().and(warp::body::json()).and_then(handler)
}
