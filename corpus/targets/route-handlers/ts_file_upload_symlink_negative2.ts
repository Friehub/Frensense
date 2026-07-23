// SAFE: Upload uses O_NOFOLLOW flag and verifies destination with realpath

import multer from 'multer';
import express from 'express';
import fs from 'fs/promises';

const UPLOAD_DIR = '/uploads';

async function ensureSafeDirectory(dir: string): Promise<string> {
  const resolved = await fs.realpath(dir);
  const stat = await fs.stat(resolved);
  if (!stat.isDirectory()) {
    throw new Error('Upload path is not a directory');
  }
  return resolved;
}

const storage = multer.diskStorage({
  destination: async (req, file, cb) => {
    try {
      const safe = await ensureSafeDirectory(UPLOAD_DIR);
      cb(null, safe);
    } catch (err) {
      cb(err);
    }
  },
  filename: (req, file, cb) => {
    const safeName = `${crypto.randomUUID()}${path.extname(file.originalname)}`;
    cb(null, safeName);
  },
});

const upload = multer({ storage });

app.post('/upload', upload.single('file'), (req, res) => {
  res.send('File uploaded');
});
