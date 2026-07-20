use serde::Serialize;
use warp::Filter;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub email: String,
}

pub async fn list_users() -> Result<impl warp::Reply, warp::Rejection> {
    let users = vec![UserResponse {
        id: 1,
        email: "admin@example.com".into(),
    }];
    Ok(warp::reply::json(&users))
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users").and(warp::get()).and_then(list_users)
}
