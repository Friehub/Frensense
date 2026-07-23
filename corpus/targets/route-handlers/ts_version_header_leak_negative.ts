// SAFE: X-Powered-By disabled, no version headers
import express from 'express';

const app = express();
app.disable('x-powered-by');

app.get('/api/health', (req, res) => {
  res.json({ status: 'ok' });
});

// SAFE: no version in custom headers
app.use((req, res, next) => {
  next();
});
