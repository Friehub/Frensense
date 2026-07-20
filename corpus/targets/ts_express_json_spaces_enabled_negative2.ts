// SAFE: Override the json replacer to strip internal fields and keep spaces disabled in production.

import express from 'express';

const app = express();
app.set('json spaces', 0);
app.set('json replacer', (key: string, value: unknown) => {
  if (key === 'internalToken' || key === 'ssn') {
    return undefined;
  }
  return value;
});

app.get('/api/users/:id', (req, res) => {
  const user = { id: req.params.id, name: 'Alice', ssn: '123-45-6789', internalToken: 'sk-abc123' };
  res.json(user);
});
