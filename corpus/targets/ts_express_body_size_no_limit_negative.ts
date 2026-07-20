// SAFE: Body parser has a size limit of 1MB to prevent memory exhaustion

import express from 'express';

const app = express();
app.use(express.json({ limit: '1mb' }));

app.post('/api/data', (req, res) => {
  const data = req.body;
  res.json({ received: Object.keys(data).length });
});
