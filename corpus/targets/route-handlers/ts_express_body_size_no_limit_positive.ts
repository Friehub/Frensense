// [frensense]
// observation: The Express body-parser middleware is configured without a size limit, allowing arbitrarily large payloads.
// impact: An attacker can send a very large request body to exhaust server memory (DoS).
// improvement: Set a reasonable limit on body-parser, e.g., express.json({ limit: '1mb' }).

import express from 'express';

const app = express();
app.use(express.json());

app.post('/api/data', (req, res) => {
  const data = req.body;
  res.json({ received: Object.keys(data).length });
});
