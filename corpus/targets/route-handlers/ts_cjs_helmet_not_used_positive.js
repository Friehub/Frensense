// [frensense]
// observation: The Express application does not use the helmet() middleware, so security headers like X-Frame-Options, X-Content-Type-Options, Strict-Transport-Security, and X-XSS-Protection are not set.
// impact: The application is vulnerable to clickjacking (missing X-Frame-Options), MIME-type sniffing attacks (missing X-Content-Type-Options), and protocol downgrade attacks (missing HSTS).
// improvement: Add app.use(require('helmet')()) as the first middleware to automatically set secure HTTP headers.

const express = require('express');

const app = express();

app.get('/api/login', function(req, res) {
  res.send('<form action="/login" method="POST"><input name="pw" type="password"><button>Login</button></form>');
});
