// SAFE: Structured logging that redacts PII fields

var express = require('express');
var app = express();

var PII_FIELDS = ['password', 'ssn', 'creditCard', 'token', 'secret', 'authorization'];

function sanitizeForLog(obj) {
  var clean = {};
  for (var key in obj) {
    if (PII_FIELDS.indexOf(key.toLowerCase()) !== -1) {
      clean[key] = '[REDACTED]';
    } else {
      clean[key] = obj[key];
    }
  }
  return clean;
}

function registerUser(req, res) {
  console.log('[AUDIT] User registration:', JSON.stringify(sanitizeForLog(req.body)));
  res.json({ created: true });
}

function handleApiCall(req, res) {
  console.log('[API]', req.method, req.path, JSON.stringify(sanitizeForLog(req.headers)));
  res.json({ ok: true });
}

app.post('/register', registerUser);
app.all('/api/*', handleApiCall);
