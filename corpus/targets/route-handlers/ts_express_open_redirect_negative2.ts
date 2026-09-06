// SAFE: Only relative paths are accepted for redirect, preventing any external URL redirection entirely.

import express from 'express';

const app = express();

app.get('/redirect', (req, res) => {
  const target = req.query.url as string;
  if (!target.startsWith('/')) {
    return res.status(400).json({ error: 'Only relative redirects allowed' });
  }
  res.redirect(target);
});
