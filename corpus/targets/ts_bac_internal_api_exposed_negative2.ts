// SAFE: Internal API uses mutual TLS (mTLS) and runs on separate port
import https from 'https';
import express from 'express';

const app = express();

app.get('/health', async (req, res) => {
  res.json({ status: 'ok' });
});

app.get('/users/:id/profile', async (req, res) => {
  if (!req.client.authorized) return res.status(401).json({ error: 'mTLS required' });
  const user = await db.prepare('SELECT * FROM users WHERE id = ?').bind(req.params.id).first();
  res.json(user);
});

const server = https.createServer({ key: INTERNAL_KEY, cert: INTERNAL_CERT, ca: INTERNAL_CA, requestCert: true, rejectUnauthorized: true }, app);
server.listen(3001, 'localhost');
