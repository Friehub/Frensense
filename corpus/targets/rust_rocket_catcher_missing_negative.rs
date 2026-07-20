use rocket::{catch, catchers, get};

#[catch(404)]
pub fn not_found() -> &'static str {
    "Resource not found."
}

#[catch(500)]
pub fn internal_error() -> &'static str {
    "Internal server error."
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
