// SAFE: Limiting both input dimensions and output dimensions with sequential_read_stream for streaming processing

import sharp from 'sharp';
import express from 'express';
import multer from 'multer';

const app = express();
const upload = multer({
  storage: multer.memoryStorage(),
  limits: { fileSize: 50 * 1024 * 1024 },
});

const MAX_DIMENSION = 8192;

app.post('/upload', upload.single('image'), async (req, res) => {
  const buffer = req.file.buffer;

  const metadata = await sharp(buffer).metadata();
  if (metadata.width > MAX_DIMENSION || metadata.height > MAX_DIMENSION) {
    return res.status(400).send(`Image dimensions exceed ${MAX_DIMENSION}x${MAX_DIMENSION}`);
  }

  if (!metadata.width || !metadata.height) {
    return res.status(400).send('Could not read image dimensions');
  }

  const resized = await sharp(buffer, { sequentialRead: true })
    .resize(2048, 2048, { fit: 'inside', withoutEnlargement: true })
    .jpeg({ quality: 85 })
    .toBuffer();

  await fs.promises.writeFile(`/uploads/${req.file.originalname}`, resized);
  res.send('ok');
});
