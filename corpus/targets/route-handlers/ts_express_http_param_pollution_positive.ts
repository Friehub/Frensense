// [frensense]
// observation: The Express app uses req.query for multiple values without handling HTTP parameter pollution (HPP). When multiple params with the same name are sent, Express parses them into an array, but the code assumes a string, causing type confusion and bypass of validation.
// impact: An attacker can send duplicate query parameters to bypass input validation or cause unexpected behavior, potentially leading to SQL injection or authorization bypass.
// improvement: Use a library like hpp() to handle duplicate parameters, or explicitly handle arrays in query parameter parsing.

import express from 'express';

const app = express();

app.get('/api/users', (req, res) => {
  const role = req.query.role as string;
  if (role !== 'admin') {
    return res.json({ users: [{ id: 1, name: 'Alice' }] });
  }
  res.json({ users: [{ id: 1, name: 'Alice', ssn: '123-45-6789' }] });
});
