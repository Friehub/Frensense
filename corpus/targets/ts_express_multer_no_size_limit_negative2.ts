// SAFE: A custom middleware checks Content-Length before multer processes the upload, rejecting large payloads early.

import express from 'express';
import multer from 'multer';
import path from 'path';

const app = express();
const maxSize = 5 * 1024 * 1024;

app.use((req, res, next) => {
  if (req.method === 'POST' && req.path === '/api/upload') {
    const contentLength = parseInt(req.headers['content-length'] || '0', 10);
    if (contentLength > maxSize) {
      return res.status(413).json({ error: 'Payload too large' });
    }
  }
  next();
});

const upload = multer({ dest: path.join(__dirname, 'uploads') });

app.post('/api/upload', upload.single('file'), (req, res) => {
  res.json({ filename: req.file?.filename });
});
