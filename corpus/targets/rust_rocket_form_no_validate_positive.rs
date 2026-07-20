// [frensense]
// observation: Rocket form handler accepts user input via `Form<LoginForm>` without any field validation — strings are used raw.
// impact: Malformed or malicious input (XSS, injection, oversized payloads) passes through unchecked.
// improvement: Add field-length checks, regex validation, or use a validator like `validator` crate with `#[validate]`.

use rocket::form::Form;
use rocket::serde::Deserialize;

#[derive(FromForm, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[post("/login", data = "<form>")]
pub async fn login(form: Form<LoginForm>) -> String {
    format!("Welcome back, {}!", form.username)
}
