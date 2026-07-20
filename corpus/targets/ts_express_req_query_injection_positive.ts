// [frensense]
// observation: The code uses `req.query.id` directly in a SQL query without validation or parameterization. An attacker can inject SQL via the query string parameter.
// impact: SQL injection — attacker can read, modify, or delete arbitrary data from the database.
// improvement: Never concatenate user input into SQL strings. Use parameterized queries (prepared statements) or an ORM with safe query builders.

import express from 'express';
import { createConnection } from 'mysql2';

const app = express();
const db = createConnection({ host: 'localhost', user: 'root', database: 'test' });

app.get('/api/users', (req, res) => {
  const id = req.query.id;
  db.query(`SELECT * FROM users WHERE id = ${id}`, (err, results) => {
    res.json(results);
  });
});
