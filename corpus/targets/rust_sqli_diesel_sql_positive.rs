// [frensense]
// observation: diesel::sql_query is called with a format!() string that includes unsanitized user input, enabling SQL injection.
// impact: An attacker can manipulate the raw SQL query to read or modify arbitrary data in the database.
// improvement: Use parameterized queries via diesel's sql_query with bind() instead of format!().

use diesel::sql_query;
use diesel::pg::PgConnection;

fn find_user_by_email(conn: &mut PgConnection, email: &str) -> Result<Vec<User>, diesel::result::Error> {
    let query = format!("SELECT * FROM users WHERE email = '{}'", email);
    sql_query(query).load::<User>(conn)
}

fn update_user_role(conn: &mut PgConnection, user_id: &str, role: &str) -> Result<(), diesel::result::Error> {
    let query = format!("UPDATE users SET role = '{}' WHERE id = '{}'", role, user_id);
    sql_query(query).execute(conn)?;
    Ok(())
}
