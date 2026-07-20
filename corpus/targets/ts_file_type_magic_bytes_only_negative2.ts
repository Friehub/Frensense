// SAFE: Uses file-type library to detect the actual MIME type from content

import multer from 'multer';
import express from 'express';
import path from 'path';
import { fileTypeFromFile } from 'file-type';
import fs from 'fs/promises';

const ALLOWED_MIMES = new Set(['image/jpeg', 'image/png', 'image/gif', 'image/webp']);

const storage = multer.diskStorage({
  destination: (req, file, cb) => cb(null, '/tmp'),
  filename: (req, file, cb) => cb(null, `${Date.now()}_${file.originalname}`),
});

const upload = multer({ storage });

app.post('/upload', upload.single('image'), async (req, res) => {
  const type = await fileTypeFromFile(req.file.path);
  if (!type || !ALLOWED_MIMES.has(type.mime)) {
    await fs.unlink(req.file.path);
    return res.status(400).send('Invalid file type');
  }

  const ext = '.' + type.ext;
  const newPath = `/uploads/${Date.now()}${ext}`;
  await fs.rename(req.file.path, newPath);
  res.send('File uploaded');
});
