// SAFE: Redirect URL is validated against an allowlist of trusted domains, preventing open redirect to external malicious sites.

import express from 'express';

const app = express();
const ALLOWED_HOSTS = ['example.com', 'app.example.com'];

app.get('/redirect', (req, res) => {
  const target = req.query.url as string;
  try {
    const url = new URL(target, 'https://example.com');
    if (!ALLOWED_HOSTS.includes(url.hostname)) {
      return res.status(400).json({ error: 'Redirect not allowed' });
    }
    res.redirect(url.toString());
  } catch {
    res.status(400).json({ error: 'Invalid URL' });
  }
});
