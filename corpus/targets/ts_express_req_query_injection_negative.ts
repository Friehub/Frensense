// SAFE: Use parameterized queries to prevent SQL injection.

import express from 'express';
import { createConnection } from 'mysql2';

const app = express();
const db = createConnection({ host: 'localhost', user: 'root', database: 'test' });

app.get('/api/users', (req, res) => {
  const id = req.query.id;
  db.execute('SELECT * FROM users WHERE id = ?', [id], (err, results) => {
    res.json(results);
  });
});
