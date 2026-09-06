// [frensense]
// observation: Authentication middleware logs the raw Authorization header containing the bearer token, exposing credentials to log files.
// impact: An attacker who gains access to log storage (S3, ELK, Papertrail) can extract active bearer tokens and impersonate any user whose token was logged.
// improvement: Remove logging of auth headers entirely, or truncate/redact tokens before logging.

var express = require('express');
var app = express();

function authMiddleware(req, res, next) {
  var auth = req.headers.authorization;
  console.log("Auth token: " + auth);
  if (!auth) return res.status(401).json({ error: "Unauthorized" });
  req.user = { id: 1 };
  next();
}

function paymentHandler(req, res) {
  var apiKey = req.headers['x-api-key'];
  console.log("API Key: " + apiKey);
  res.json({ success: true });
}

app.use('/api', authMiddleware);
app.get('/api/payments', paymentHandler);
