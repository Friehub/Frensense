// SAFE: Body parser size limit is set via a config variable for consistent policy across routes

import express from 'express';

const app = express();
const MAX_PAYLOAD_SIZE = '100kb';
app.use(express.json({ limit: MAX_PAYLOAD_SIZE }));
app.use(express.urlencoded({ extended: true, limit: MAX_PAYLOAD_SIZE }));

app.post('/api/data', (req, res) => {
  const data = req.body;
  res.json({ received: Object.keys(data).length });
});

app.post('/api/form', (req, res) => {
  res.json({ name: req.body.name });
});
