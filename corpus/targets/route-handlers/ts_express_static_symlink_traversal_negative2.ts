// SAFE: Validate that the resolved path stays within the public directory by checking the real path.

import express from 'express';
import path from 'path';
import fs from 'fs';

const app = express();
const publicDir = path.resolve(__dirname, 'public');

app.use('/static', (req, res, next) => {
  const requestedPath = path.join(publicDir, req.path);
  const realPath = fs.realpathSync(requestedPath);
  if (!realPath.startsWith(publicDir)) {
    return res.status(403).send('Forbidden');
  }
  express.static(publicDir)(req, res, next);
});

app.get('/api/health', (req, res) => {
  res.json({ status: 'ok' });
});
