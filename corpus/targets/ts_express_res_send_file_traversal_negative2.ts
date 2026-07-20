// SAFE: Use a whitelist of allowed filenames instead of accepting arbitrary paths.

import express from 'express';

const ALLOWED_FILES = new Set(['report.pdf', 'invoice.pdf', 'terms.pdf']);

const app = express();

app.get('/files/:file', (req, res) => {
  const fileName = req.params.file;
  if (!ALLOWED_FILES.has(fileName)) {
    res.status(403).json({ error: 'Forbidden' });
    return;
  }
  res.sendFile(`/var/www/uploads/${fileName}`);
});
