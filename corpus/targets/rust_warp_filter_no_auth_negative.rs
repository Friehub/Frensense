use warp::Filter;
use std::collections::HashSet;

fn with_auth() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::header::<String>("authorization")
        .and_then(|token: String| async move {
            if token == "Bearer valid-token" {
                Ok(token)
            } else {
                Err(warp::reject::reject())
            }
        })
}

pub async fn admin_handler(usr: String) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("admin panel for {}", usr))
}

pub fn admin_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("admin" / ..)
        .and(warp::get())
        .and(with_auth())
        .and_then(admin_handler)
}
