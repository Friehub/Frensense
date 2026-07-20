// SAFE: Header values are validated to strip CRLF characters, preventing response splitting injection.

import express from 'express';

const app = express();
app.use(express.json());

function sanitizeHeaderValue(value: string): string {
  return value.replace(/[\r\n]/g, '').trim();
}

app.post('/api/set-header', (req, res) => {
  const headerName = req.body.name;
  const headerValue = sanitizeHeaderValue(req.body.value);
  res.set(headerName, headerValue);
  res.json({ ok: true });
});
