// SAFE: Security headers are set manually without helmet, giving explicit control over each header value.

import express from 'express';

const app = express();
app.use((req, res, next) => {
  res.setHeader('X-Frame-Options', 'DENY');
  res.setHeader('X-Content-Type-Options', 'nosniff');
  res.setHeader('Strict-Transport-Security', 'max-age=63072000; includeSubDomains');
  res.setHeader('X-XSS-Protection', '0');
  res.setHeader('Referrer-Policy', 'strict-origin-when-cross-origin');
  next();
});

app.get('/api/login', (req, res) => {
  res.send('<form action="/login" method="POST"><input name="pw" type="password"><button>Login</button></form>');
});
