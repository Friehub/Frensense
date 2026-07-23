// [frensense]
// observation: "File type validation only checks the file extension, not the actual content (magic bytes)."
// impact: "An attacker can upload a malicious file (e.g., a PHP web shell) with a .jpg extension, bypassing extension-only checks. The file can then be executed if the server processes it based on content type."
// improvement: "Always validate the file type by reading magic bytes (file signatures) in addition to extension checks."

import multer from 'multer';
import express from 'express';
import path from 'path';

const app = express();

const ALLOWED_EXTENSIONS = ['.jpg', '.jpeg', '.png', '.gif'];

const storage = multer.diskStorage({
  destination: (req, file, cb) => {
    cb(null, '/uploads');
  },
  filename: (req, file, cb) => {
    const ext = path.extname(file.originalname).toLowerCase();
    if (!ALLOWED_EXTENSIONS.includes(ext)) {
      return cb(new Error('Invalid extension'));
    }
    cb(null, `${Date.now()}${ext}`);
  },
});

const upload = multer({ storage });

app.post('/upload', upload.single('image'), (req, res) => {
  res.send('File uploaded');
});
