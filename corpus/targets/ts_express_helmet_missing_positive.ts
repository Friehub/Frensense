// [frensense]
// observation: The Express app does not use the helmet middleware, so security headers like X-Frame-Options, X-Content-Type-Options, and Strict-Transport-Security are not set on responses.
// impact: An attacker can perform clickjacking (missing X-Frame-Options), MIME-type sniffing (missing X-Content-Type-Options), and downgrade attacks (missing HSTS).
// improvement: Add app.use(helmet()) as the first middleware to set secure HTTP headers automatically.

import express from 'express';

const app = express();

app.get('/api/login', (req, res) => {
  res.send('<form action="/login" method="POST"><input name="pw" type="password"><button>Login</button></form>');
});
