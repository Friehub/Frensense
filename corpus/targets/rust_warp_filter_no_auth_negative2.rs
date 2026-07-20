// SAFE: JWT verification filter gates the admin route.
use warp::Filter;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

fn with_jwt() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::header::<String>("authorization")
        .and_then(|token: String| async move {
            let parts: Vec<&str> = token.splitn(2, ' ').collect();
            if parts.len() != 2 || parts[0] != "Bearer" {
                return Err(warp::reject::reject());
            }
            let data = decode::<serde_json::Value>(
                parts[1],
                &DecodingKey::from_secret(b"secret"),
                &Validation::new(Algorithm::HS256),
            ).map_err(|_| warp::reject::reject())?;
            Ok(data.claims.to_string())
        })
}

pub async fn admin_handler(sub: String) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("hello {}", sub))
}

pub fn admin_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("admin" / ..)
        .and(warp::get())
        .and(with_jwt())
        .and_then(admin_handler)
}
