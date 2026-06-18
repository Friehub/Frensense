// Vulnerable: SQL injection
const query = `SELECT * FROM users WHERE id = ${userId}`;
db.query(query);
