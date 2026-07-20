// [frensense]
// observation: The Express app sets 'json spaces' to a value greater than 0 in production, causing JSON responses to be pretty-printed with whitespace and potentially leaking internal data in error responses.
// impact: Pretty-printed JSON responses can leak stack traces, internal paths, and query details in error responses, aiding an attacker's reconnaissance.
// improvement: Set 'json spaces' to 0 in production, or only enable it in development with process.env.NODE_ENV.

import express from 'express';

const app = express();
app.set('json spaces', 2);

app.get('/api/users/:id', (req, res) => {
  const user = { id: req.params.id, name: 'Alice' };
  res.json(user);
});
