// SAFE: Custom validation logic applied to each field before use.
use rocket::form::Form;
use rocket::serde::Deserialize;

#[derive(FromForm, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

fn validate_login(form: &LoginForm) -> Result<(), &'static str> {
    if form.username.is_empty() || form.username.len() > 64 {
        return Err("invalid username");
    }
    if form.password.len() < 8 {
        return Err("password too short");
    }
    Ok(())
}

#[post("/login", data = "<form>")]
pub async fn login(form: Form<LoginForm>) -> Result<String, String> {
    validate_login(&form).map_err(|e| e.to_string())?;
    Ok(format!("Welcome back, {}!", form.username))
}
