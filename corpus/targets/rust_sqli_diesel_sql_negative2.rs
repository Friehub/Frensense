// SAFE: Uses Diesel's typed query DSL instead of raw SQL
use diesel::prelude::*;
use diesel::pg::PgConnection;

fn find_user_by_email(conn: &mut PgConnection, email: &str) -> Result<Vec<User>, diesel::result::Error> {
    users::table
        .filter(users::email.eq(email))
        .load::<User>(conn)
}

fn update_user_role(conn: &mut PgConnection, user_id: &str, new_role: &str) -> Result<(), diesel::result::Error> {
    diesel::update(users::table.find(user_id))
        .set(users::role.eq(new_role))
        .execute(conn)?;
    Ok(())
}
