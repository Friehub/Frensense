// SAFE: Image metadata is read first to validate dimensions before processing

import sharp from 'sharp';
import express from 'express';
import multer from 'multer';

const app = express();
const upload = multer({
  storage: multer.memoryStorage(),
  limits: { fileSize: 10 * 1024 * 1024 },
});

const MAX_WIDTH = 4096;
const MAX_HEIGHT = 4096;

app.post('/upload', upload.single('image'), async (req, res) => {
  const buffer = req.file.buffer;
  const metadata = await sharp(buffer).metadata();

  if (metadata.width > MAX_WIDTH || metadata.height > MAX_HEIGHT) {
    return res.status(400).send('Image dimensions exceed limit');
  }

  const resized = await sharp(buffer)
    .resize(2048, 2048, { fit: 'inside' })
    .jpeg({ quality: 85 })
    .toBuffer();

  await fs.promises.writeFile(`/uploads/${req.file.originalname}`, resized);
  res.send('ok');
});
