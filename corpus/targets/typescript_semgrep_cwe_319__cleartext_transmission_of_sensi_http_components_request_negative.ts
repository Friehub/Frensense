// Fixed: parameterized query
db.query('SELECT * FROM users WHERE id = $1', [userId]);
