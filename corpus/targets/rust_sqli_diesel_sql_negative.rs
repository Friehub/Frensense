use diesel::sql_query;
use diesel::pg::PgConnection;

fn find_user_by_email(conn: &mut PgConnection, email: &str) -> Result<Vec<User>, diesel::result::Error> {
    sql_query("SELECT * FROM users WHERE email = $1")
        .bind::<diesel::sql_types::Text, _>(email)
        .load::<User>(conn)
}

fn update_user_role(conn: &mut PgConnection, user_id: &str, role: &str) -> Result<(), diesel::result::Error> {
    sql_query("UPDATE users SET role = $1 WHERE id = $2")
        .bind::<diesel::sql_types::Text, _>(role)
        .bind::<diesel::sql_types::Text, _>(user_id)
        .execute(conn)?;
    Ok(())
}
