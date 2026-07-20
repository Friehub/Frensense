// SAFE: Manual mapping to response struct excludes sensitive fields.
use serde::Serialize;
use warp::Filter;

struct User {
    id: u64,
    email: String,
    password_hash: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub email: String,
}

fn to_response(user: &User) -> UserResponse {
    UserResponse {
        id: user.id,
        email: user.email.clone(),
    }
}

pub async fn list_users() -> Result<impl warp::Reply, warp::Rejection> {
    let users = vec![User {
        id: 1,
        email: "admin@example.com".into(),
        password_hash: "$2b$12$abcdefghijklmnop".into(),
    }];
    let resp: Vec<UserResponse> = users.iter().map(to_response).collect();
    Ok(warp::reply::json(&resp))
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users").and(warp::get()).and_then(list_users)
}
