// [frensense]
// observation: The login endpoint has no rate limiting applied via express-rate-limit, allowing unlimited login attempts.
// impact: An attacker can brute-force user passwords without any throttling, leading to account takeover.
// improvement: Apply express-rate-limit middleware to the login route to limit failed attempts, e.g., 5 attempts per 15 minutes.

import express from 'express';

const app = express();
app.use(express.json());

app.post('/api/login', (req, res) => {
  const { username, password } = req.body;
  if (username === 'admin' && password === 's3cret') {
    res.json({ token: 'valid-session-token' });
  } else {
    res.status(401).json({ error: 'Invalid credentials' });
  }
});
