// SAFE: Debug endpoints are only enabled in non-production environments
import express from 'express';

const app = express();

if (process.env.NODE_ENV !== 'production') {
  app.get('/debug/env', (req, res) => {
    res.json(process.env);
  });
}

app.get('/health', (req, res) => {
  res.json({ status: 'ok' });
});
