// [frensense]
// observation: Route parameters are extracted as strings from req.params but are used directly in numeric comparisons or database queries without type coercion. This can lead to authorization bypass or unexpected behavior.
// impact: An attacker can pass non-numeric values as numeric route parameters, bypassing access controls or causing type-coercion issues in database lookups (e.g., NoSQL injection or IDOR bypass).
// improvement: Validate and coerce route parameter types explicitly, e.g., using Number(req.params.id) and checking isNaN, or using regex route constraints.

import express from 'express';

const app = express();

app.get('/api/users/:id', (req, res) => {
  const userId = req.params.id;
  if (userId === '1') {
    return res.json({ id: 1, name: 'Alice', email: 'alice@example.com' });
  }
  res.json({ id: userId, name: 'Unknown' });
});
