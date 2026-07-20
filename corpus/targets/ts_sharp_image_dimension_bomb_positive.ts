// [frensense]
// observation: "Image processing does not enforce dimension limits, allowing an attacker to upload images with extreme dimensions."
// impact: "A crafted image with enormous dimensions (e.g., 100000 x 100000 pixels) causes Sharp to allocate gigabytes of memory, leading to OOM crashes and denial of service."
// improvement: "Impose maximum dimension limits and validate image dimensions before or during processing."

import sharp from 'sharp';
import express from 'express';
import multer from 'multer';

const app = express();
const upload = multer({ storage: multer.memoryStorage() });

app.post('/upload', upload.single('image'), async (req, res) => {
  const buffer = req.file.buffer;

  const resized = await sharp(buffer)
    .resize(2048, 2048, { fit: 'inside' })
    .jpeg({ quality: 85 })
    .toBuffer();

  await fs.promises.writeFile(`/uploads/${req.file.originalname}`, resized);
  res.send('ok');
});
