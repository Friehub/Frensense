// [frensense]
// observation: Rocket application registers routes but no error catchers for 404 or 500, so errors return the default Rocket HTML page which may leak internal state.
// impact: Information disclosure — stack traces, file paths, or version details may be exposed on error.
// improvement: Register custom error catchers for 404 and 500 that return sanitized responses.

use rocket::get;

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello!"
}

pub fn rocket() -> rocket::Rocket< rocket::Build> {
    rocket::build().mount("/", routes![hello])
}
