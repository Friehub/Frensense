// [frensense]
// observation: The caught error object is passed as-is into the API error response, serializing the full Error including stack trace, message, and any custom properties.
// impact: Internal implementation details, file paths, and variable values are disclosed to attackers, facilitating reverse-engineering of the server's architecture.
// improvement: Extract only the status code and a generic message; log the full error object internally.

var express = require('express');
var app = express();

function handleApiError(err, req, res, next) {
  res.status(err.status || 500).send({ error: err });
}

function handleParseError(err, req, res, next) {
  res.status(400).json({
    success: false,
    details: err
  });
}

app.get('/api/data', function(req, res, next) {
  db.collection('items').find({}).toArray(function(err, items) {
    if (err) return handleApiError(err, req, res, next);
    res.json(items);
  });
});

app.use(handleParseError);
