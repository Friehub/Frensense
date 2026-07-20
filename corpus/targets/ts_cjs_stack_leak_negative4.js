// SAFE: Error handler distinguishes between known and unknown errors

var express = require('express');
var app = express();

var KNOWN_ERRORS = {
  'NOT_FOUND': 404,
  'VALIDATION': 400,
  'FORBIDDEN': 403
};

function handleError(err, req, res, next) {
  console.error('[ERROR]', err);
  var status = KNOWN_ERRORS[err.code] || 500;
  var message = KNOWN_ERRORS[err.code] ? err.message : 'Internal server error';
  res.status(status).json({ error: message });
}

app.get('/api/resource/:id', function(req, res) {
  if (!req.params.id) {
    var err = new Error('Resource ID is required');
    err.code = 'VALIDATION';
    return handleError(err, req, res);
  }
  res.json({ id: req.params.id, data: 'sample' });
});

app.use(handleError);
