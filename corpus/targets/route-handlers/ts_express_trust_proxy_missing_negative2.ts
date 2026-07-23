// SAFE: trust proxy is configured with the exact number of proxy hops for stricter security

import express from 'express';

const app = express();
app.set('trust proxy', 1);

app.get('/api/users', (req, res) => {
  const clientIp = req.ip;
  res.json({ ip: clientIp });
});
