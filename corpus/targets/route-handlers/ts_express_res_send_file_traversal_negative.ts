// SAFE: Resolve and check that the resulting path stays within the allowed base directory.

import express from 'express';
import { resolve, normalize } from 'node:path';

const UPLOADS_DIR = '/var/www/uploads';

const app = express();

app.get('/files/:file', (req, res) => {
  const requested = normalize(req.params.file);
  const fullPath = resolve(UPLOADS_DIR, requested);
  if (!fullPath.startsWith(UPLOADS_DIR)) {
    res.status(403).json({ error: 'Forbidden' });
    return;
  }
  res.sendFile(fullPath);
});
