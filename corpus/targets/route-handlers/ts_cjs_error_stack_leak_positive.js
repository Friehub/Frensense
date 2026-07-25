// [frensense]
// observation: There is no custom error-handling middleware, so Express's default error handler sends the full stack trace to the client when an error occurs.
// impact: An attacker can trigger an error to leak sensitive information including file paths, module versions, application structure, and internal logic via the stack trace.
// improvement: Add a 4-argument error handler that returns a generic error message and logs the stack internally.
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium

const express = require('express');

const app = express();

app.get('/api/users/:id', function(req, res) {
  var user = getUserById(parseInt(req.params.id, 10));
  res.json(user);
});

function getUserById(id) {
  if (id <= 0) {
    throw new Error('Invalid user ID: ' + id + ' — stack: ' + new Error().stack);
  }
  return { name: 'Alice' };
}
