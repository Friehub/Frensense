// SAFE: Uses a UUID-based filename and forces JPEG output regardless of input extension

import sharp from 'sharp';
import express from 'express';
import multer from 'multer';
import { v4 as uuidv4 } from 'uuid';

const app = express();
const upload = multer({ dest: '/tmp/uploads/' });

app.post('/upload', upload.single('image'), async (req, res) => {
  if (!req.file) {
    return res.status(400).send('No file uploaded');
  }

  const safeName = `${uuidv4()}.jpg`;

  await sharp(req.file.path)
    .resize(800, 600)
    .jpeg({ quality: 85 })
    .toFile(path.join('/uploads', safeName));

  res.json({ url: `/uploads/${safeName}` });
});
