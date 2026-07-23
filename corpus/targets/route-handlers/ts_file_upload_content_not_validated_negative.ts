// SAFE: re-encode image to strip embedded content
import multer from 'multer';
import sharp from 'sharp';

const upload = multer({ dest: 'uploads/' });

app.post('/api/upload', upload.single('file'), async (req, res) => {
  try {
    // SAFE: sharp validates and re-encodes — strips any injected content
    const processed = await sharp(req.file.path)
      .resize(800, 800, { fit: 'inside' })
      .jpeg({ quality: 80 })
      .toBuffer();
    const safePath = `uploads/${crypto.randomUUID()}.jpg`;
    await fs.writeFile(safePath, processed);
    await fs.unlink(req.file.path);
    res.json({ filename: path.basename(safePath) });
  } catch (err) {
    await fs.unlink(req.file.path).catch(() => {});
    res.status(400).json({ error: 'Invalid image content' });
  }
});
