// SAFE: Multer is configured with a fileSize limit of 5 MB to prevent resource exhaustion.

import express from 'express';
import multer from 'multer';
import path from 'path';

const app = express();
const upload = multer({
  dest: path.join(__dirname, 'uploads'),
  limits: { fileSize: 5 * 1024 * 1024 },
});

app.post('/api/upload', upload.single('file'), (req, res) => {
  res.json({ filename: req.file?.filename });
});
