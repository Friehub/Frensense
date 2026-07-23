// [frensense]
// observation: Multer is configured without a fileSize limit, allowing clients to upload arbitrarily large files and exhaust server disk space or memory.
// impact: An attacker can upload enormous files to fill the disk, causing denial of service (DoS) or crashing the application.
// improvement: Set a fileSize limit in the multer configuration, e.g., limits: { fileSize: 5 * 1024 * 1024 }.

import express from 'express';
import multer from 'multer';
import path from 'path';

const app = express();
const upload = multer({ dest: path.join(__dirname, 'uploads') });

app.post('/api/upload', upload.single('file'), (req, res) => {
  res.json({ filename: req.file?.filename });
});
