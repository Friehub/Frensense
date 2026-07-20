// SAFE: Custom catchers registered that return JSON error responses without internal detail.
use rocket::{catch, catchers, get};
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorBody {
    pub error: String,
}

#[catch(404)]
pub fn not_found() -> Json<ErrorBody> {
    Json(ErrorBody { error: "not_found".into() })
}

#[catch(500)]
pub fn internal_error() -> Json<ErrorBody> {
    Json(ErrorBody { error: "internal_error".into() })
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello!"
}

pub fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![hello])
        .register("/", catchers![not_found, internal_error])
}
