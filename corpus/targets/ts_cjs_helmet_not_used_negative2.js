// SAFE: Security headers are set manually via middleware providing explicit control over each header.

const express = require('express');

const app = express();
app.use(function(req, res, next) {
  res.setHeader('X-Frame-Options', 'DENY');
  res.setHeader('X-Content-Type-Options', 'nosniff');
  res.setHeader('Strict-Transport-Security', 'max-age=63072000; includeSubDomains');
  res.setHeader('X-XSS-Protection', '0');
  res.setHeader('Referrer-Policy', 'strict-origin-when-cross-origin');
  next();
});

app.get('/api/login', function(req, res) {
  res.send('<form action="/login" method="POST"><input name="pw" type="password"><button>Login</button></form>');
});
