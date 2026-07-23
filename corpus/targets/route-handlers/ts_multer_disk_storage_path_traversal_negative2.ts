// SAFE: Uses a UUID-based filename instead of the user-provided name

import multer from 'multer';
import express from 'express';
import path from 'path';
import { v4 as uuidv4 } from 'uuid';

const app = express();

const storage = multer.diskStorage({
  destination: (req, file, cb) => {
    cb(null, '/uploads');
  },
  filename: (req, file, cb) => {
    const ext = path.extname(file.originalname);
    cb(null, `${uuidv4()}${ext}`);
  },
});

const upload = multer({
  storage,
  limits: { fileSize: 5 * 1024 * 1024 },
});

app.post('/upload', upload.single('file'), (req, res) => {
  res.send('File uploaded');
});
