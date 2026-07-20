// [frensense]
// observation: Warp reply handler returns internal data structures (e.g., `Vec<User>` with password hashes) directly as JSON without sanitization.
// impact: Sensitive fields like password hashes, internal IDs, or PII are leaked to the client.
// improvement: Define a response DTO that excludes sensitive fields before serialization.

use serde::Serialize;
use warp::Filter;

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub password_hash: String,
}

pub async fn list_users() -> Result<impl warp::Reply, warp::Rejection> {
    let users = vec![User {
        id: 1,
        email: "admin@example.com".into(),
        password_hash: "$2b$12$abcdefghijklmnop".into(),
    }];
    Ok(warp::reply::json(&users))
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users").and(warp::get()).and_then(list_users)
}
