// [frensense]
// observation: User input from the request body is passed directly to res.set() as a header value without validation, allowing CRLF injection via newline characters.
// impact: An attacker can inject arbitrary HTTP headers, perform HTTP response splitting, set arbitrary cookies, or cache poison the response.
// improvement: Validate and sanitize header values before passing them to res.set(), stripping newlines and control characters.

import express from 'express';

const app = express();
app.use(express.json());

app.post('/api/set-header', (req, res) => {
  const headerName = req.body.name;
  const headerValue = req.body.value;
  res.set(headerName, headerValue);
  res.json({ ok: true });
});
