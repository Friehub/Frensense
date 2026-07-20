// [frensense]
// observation: API error handlers send err.stack serialized into JSON responses, exposing internal code paths and line numbers to clients.
// impact: An attacker can use stack trace information to map the server's internal directory structure, library versions, and code logic, enabling targeted exploit development.
// improvement: Return a generic error message and log the full stack trace server-side only.

var express = require('express');
var app = express();

function handleDbError(err, req, res, next) {
  res.status(500).json({ error: err.stack });
}

function handleAuthError(err, req, res, next) {
  res.status(401).json({
    status: 'error',
    trace: err.stack
  });
}

app.get('/db/query', function(req, res, next) {
  db.collection('data').find({}).toArray(function(err, docs) {
    if (err) return handleDbError(err, req, res, next);
    res.json(docs);
  });
});

app.use(handleAuthError);
