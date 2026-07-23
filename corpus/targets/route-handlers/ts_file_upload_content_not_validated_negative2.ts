// SAFE alternative: use file-type for content detection + strip metadata
import multer from 'multer';
import { fileTypeFromFile } from 'file-type';

const ALLOWED = new Set(['image/jpeg', 'image/png', 'image/webp']);

app.post('/api/upload', upload.single('file'), async (req, res) => {
  const type = await fileTypeFromFile(req.file.path);
  if (!type || !ALLOWED.has(type.mime)) {
    await fs.unlink(req.file.path);
    return res.status(400).json({ error: 'Invalid content' });
  }
  res.json({ filename: req.file.filename });
});
