// SAFE: The hpp() middleware is used to reject or deduplicate duplicate query parameters, preventing HTTP parameter pollution attacks.

import express from 'express';
import hpp from 'hpp';

const app = express();
app.use(hpp());

app.get('/api/users', (req, res) => {
  const role = req.query.role as string;
  if (role !== 'admin') {
    return res.json({ users: [{ id: 1, name: 'Alice' }] });
  }
  res.json({ users: [{ id: 1, name: 'Alice', ssn: '123-45-6789' }] });
});
