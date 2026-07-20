// SAFE: Output filename is sanitized and validated against an allowed extension list

import sharp from 'sharp';
import express from 'express';
import multer from 'multer';
import path from 'path';

const app = express();
const upload = multer({ dest: '/tmp/uploads/' });

const ALLOWED_EXTENSIONS = new Set(['.jpg', '.jpeg', '.png', '.webp']);

app.post('/upload', upload.single('image'), async (req, res) => {
  const ext = path.extname(req.file.originalname).toLowerCase();
  if (!ALLOWED_EXTENSIONS.has(ext)) {
    return res.status(400).send('Invalid file extension');
  }

  const safeName = `${crypto.randomUUID()}${ext}`;

  await sharp(req.file.path)
    .resize(800, 600)
    .jpeg()
    .toFile(path.join('/uploads', safeName));

  res.json({ url: `/uploads/${safeName}` });
});
