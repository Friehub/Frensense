// [frensense]
// observation: `serde_json::Value` is used as a catch-all deserialization target instead of a strongly-typed struct, allowing arbitrary JSON structures to flow through the application.
// impact: Type confusion — the application may assume fields exist that do not, leading to panics or logic errors. Attackers can inject unexpected fields for mass assignment or parameter pollution.
// improvement: Define a concrete struct with `#[derive(Deserialize)]` and use `#[serde(deny_unknown_fields)]`.

use serde_json::Value;
use warp::Filter;

pub async fn handler(body: Value) -> Result<impl warp::Reply, warp::Rejection> {
    let name = body["name"].as_str().unwrap_or("unknown");
    Ok(format!("hello {}", name))
}

pub fn route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post().and(warp::body::json()).and_then(handler)
}
