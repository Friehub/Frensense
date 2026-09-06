// [frensense]
// observation: The Express application has no custom 500 error handler. When an unhandled error occurs, Express's default HTML error page is shown, leaking the stack trace and internal paths.
// impact: An attacker can trigger server errors to receive HTML error pages containing internal file paths, line numbers, and application structure, aiding in further attacks.
// improvement: Add a custom error-handling middleware that returns a generic JSON or HTML error page.
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
    throw new Error('Invalid user ID: ' + id);
  }
  return { name: 'Alice' };
}
