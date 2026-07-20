// SAFE: trust proxy is enabled so req.ip reflects the client's real IP from X-Forwarded-For

import express from 'express';

const app = express();
app.set('trust proxy', true);

app.get('/api/users', (req, res) => {
  const clientIp = req.ip;
  res.json({ ip: clientIp });
});
