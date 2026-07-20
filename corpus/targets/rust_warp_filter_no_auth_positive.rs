// [frensense]
// observation: A warp filter chain for a sensitive route (e.g., `/admin`) is defined without any authentication or authorization layer.
// impact: Unauthenticated users can access admin endpoints and perform privileged operations.
// improvement: Add an authentication filter (e.g., `warp::filters::auth::Bearer` or a custom header check) before the route handler.

use warp::Filter;

pub async fn admin_handler() -> Result<impl warp::Reply, warp::Rejection> {
    Ok("admin panel")
}

pub fn admin_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("admin" / ..).and(warp::get()).and_then(admin_handler)
}
