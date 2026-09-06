// SAFE alternative: check Content-Length header before reading body
import multer from 'multer';
import express from 'express';

const app = express();
const MAX_SIZE = 5 * 1024 * 1024;

// SAFE: reject oversized requests at middleware level
app.use('/api/upload', (req, res, next) => {
  const length = parseInt(req.headers['content-length'] || '0', 10);
  if (length > MAX_SIZE) {
    return res.status(413).json({ error: 'File too large' });
  }
  next();
});

const upload = multer({
  dest: 'uploads/',
  limits: { fileSize: MAX_SIZE },
});
