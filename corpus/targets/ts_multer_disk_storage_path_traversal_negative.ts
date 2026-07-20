// SAFE: Filename is sanitized to remove directory traversal sequences

import multer from 'multer';
import express from 'express';
import path from 'path';

const app = express();

function sanitizeFilename(name: string): string {
  return name.replace(/\.\.\//g, '').replace(/\.\.\\/g, '').replace(/\//g, '_').replace(/\\/g, '_');
}

const storage = multer.diskStorage({
  destination: (req, file, cb) => {
    cb(null, '/uploads');
  },
  filename: (req, file, cb) => {
    cb(null, sanitizeFilename(file.originalname));
  },
});

const upload = multer({ storage });

app.post('/upload', upload.single('file'), (req, res) => {
  res.send('File uploaded');
});
