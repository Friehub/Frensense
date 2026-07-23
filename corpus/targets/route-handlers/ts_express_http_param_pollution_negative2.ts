// SAFE: Query parameters are explicitly handled to ensure role is always a single string, using Array.isArray to pick the first value.

import express from 'express';

const app = express();

function getQueryString(req: express.Request, key: string): string | undefined {
  const val = req.query[key];
  if (Array.isArray(val)) {
    return val[0] as string;
  }
  return val as string | undefined;
}

app.get('/api/users', (req, res) => {
  const role = getQueryString(req, 'role');
  if (role !== 'admin') {
    return res.json({ users: [{ id: 1, name: 'Alice' }] });
  }
  res.json({ users: [{ id: 1, name: 'Alice', ssn: '123-45-6789' }] });
});
