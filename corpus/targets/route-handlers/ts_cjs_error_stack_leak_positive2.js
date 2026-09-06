// [frensense]
// observation: There is no custom error-handling middleware, so Express's default error handler sends the full stack trace to the client when an error occurs.
// impact: An attacker can trigger an error to leak sensitive information including file paths, module versions, application structure, and internal logic via the stack trace.
// improvement: Add a 4-argument error handler that returns a generic error message and logs the stack internally.

var express = require('express');
var app = express();

function fetchProfile(req, res) {
  var profile = loadProfile(req.params.id);
  res.json(profile);
}

function loadProfile(id) {
  var num = parseInt(id, 10);
  if (isNaN(num)) {
    throw new Error('Invalid profile identifier: ' + id);
  }
  return { id: num, name: 'Alice', email: 'alice@test.com' };
}

app.get('/profile/:id', fetchProfile);

app.use(function(err, req, res, next) {
  res.status(500).json({ error: err.stack });
});
