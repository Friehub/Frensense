// [frensense]
// observation: The Express app has no custom error-handling middleware (no 4-argument handler). When an error is thrown or passed via next(err), the default Express error handler returns the full stack trace to the client.
// impact: An attacker can trigger an error to leak the full stack trace, revealing file paths, dependency versions, application structure, and internal logic.
// improvement: Add a 4-argument error handler: app.use((err, req, res, next) => { ... }) that returns a generic error message and logs internally.
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium

import express from 'express';

const app = express();

app.get('/api/users/:id', (req, res) => {
  const user = getUserById(Number(req.params.id));
  res.json(user);
});

function getUserById(id: number): { name: string } {
  if (id <= 0) {
    throw new Error(`Invalid user ID: ${id} — stack: ${new Error().stack}`);
  }
  return { name: 'Alice' };
}
