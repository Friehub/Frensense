// [frensense]
// observation: Warp CORS configuration uses `warp::cors().allow_any_origin()` which allows any website to make cross-origin requests.
// impact: CSRF attacks, data exfiltration by malicious sites, bypass of same-origin policy.
// improvement: Restrict allowed origins to specific trusted domains.

use warp::Filter;

pub fn cors_config() -> warp::cors::Builder {
    warp::cors().allow_any_origin()
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api = warp::path!("data").and(warp::get()).map(|| "data");
    api.with(cors_config())
}
