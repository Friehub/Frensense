// SAFE: The upload destination is resolved with realpath before writing, and symlinks are not followed

import multer from 'multer';
import express from 'express';
import path from 'path';
import fs from 'fs/promises';

const UPLOAD_DIR = '/uploads';

const storage = multer.diskStorage({
  destination: async (req, file, cb) => {
    const resolved = await fs.realpath(UPLOAD_DIR);
    if (!resolved.startsWith('/uploads')) {
      return cb(new Error('Invalid upload path'));
    }
    cb(null, resolved);
  },
  filename: async (req, file, cb) => {
    const safeName = `${Date.now()}_${file.originalname.replace(/[^a-zA-Z0-9._-]/g, '')}`;
    cb(null, safeName);
  },
});

const upload = multer({ storage });

app.post('/upload', upload.single('file'), (req, res) => {
  res.send('File uploaded');
});
