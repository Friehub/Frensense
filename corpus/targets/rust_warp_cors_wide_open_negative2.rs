// SAFE: CORS restricted to a specific list of allowed origin patterns.
use warp::Filter;

fn allowed_origin(origin: &str) -> bool {
    let trusted = ["https://app.example.com", "https://admin.example.com"];
    trusted.contains(&origin)
}

pub fn cors_config() -> warp::cors::Builder {
    warp::cors()
        .allow_origin("https://app.example.com")
        .allow_origin("https://admin.example.com")
        .allow_methods(vec!["GET"])
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api = warp::path!("data").and(warp::get()).map(|| "data");
    api.with(cors_config())
}
