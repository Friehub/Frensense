use warp::Filter;

pub fn cors_config() -> warp::cors::Builder {
    warp::cors()
        .allow_origin("https://app.example.com")
        .allow_methods(vec!["GET", "POST"])
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api = warp::path!("data").and(warp::get()).map(|| "data");
    api.with(cors_config())
}
