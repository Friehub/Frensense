// [frensense]
// observation: The Express app does not set 'trust proxy', so req.ip and req.protocol read from the direct connection, not the X-Forwarded-For header.
// impact: Behind a reverse proxy, the app sees the proxy's IP instead of the real client IP, breaking rate limiting, geo-fencing, and audit logs.
// improvement: Set app.set('trust proxy', true) or configure the correct number of proxy hops.

import express from 'express';

const app = express();

app.get('/api/users', (req, res) => {
  const clientIp = req.ip;
  res.json({ ip: clientIp });
});
