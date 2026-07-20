// SAFE: Only allow header values from a predefined map instead of accepting arbitrary user input.

import express from 'express';

const app = express();
app.use(express.json());

const allowedHeaders: Record<string, string> = {
  'X-Request-Id': 'request-id',
  'X-Trace-Id': 'trace-id',
};

app.post('/api/set-header', (req, res) => {
  const headerName = req.body.name as string;
  const allowedValue = allowedHeaders[headerName];
  if (!allowedValue) {
    return res.status(400).json({ error: 'Header not allowed' });
  }
  res.set(headerName, allowedValue);
  res.json({ ok: true });
});
