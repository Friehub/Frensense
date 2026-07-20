use rocket::form::Form;
use rocket::serde::Deserialize;
use validator::Validate;

#[derive(FromForm, Deserialize, Validate)]
pub struct LoginForm {
    #[validate(length(min = 1, max = 64))]
    pub username: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

#[post("/login", data = "<form>")]
pub async fn login(form: Form<LoginForm>) -> Result<String, String> {
    if form.validate().is_err() {
        return Err("validation failed".into());
    }
    Ok(format!("Welcome back, {}!", form.username))
}
