// [frensense]
// observation: Strict-Transport-Security header is missing from HTTPS responses, allowing downgrade attacks.
// impact: Without HSTS, an attacker with network access can perform SSL stripping: downgrade the user's HTTPS connection to HTTP and intercept all traffic, including credentials and session tokens.
// improvement: Set Strict-Transport-Security header with a long max-age (e.g., max-age=31536000; includeSubDomains) on all HTTPS responses.

import express from 'express';

const app = express();

// VULNERABLE: no HSTS header
app.get('/api/login', (req, res) => {
  res.json({ token: generateToken(req.body) });
});

app.get('/', (req, res) => {
  res.send('<html><body>Welcome</body></html>');
});
