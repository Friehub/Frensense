// SAFE: validate MIME type with magic bytes
import multer from 'multer';
import fs from 'fs/promises';
import { fileTypeFromFile } from 'file-type';

const ALLOWED_TYPES = ['image/jpeg', 'image/png', 'image/webp'];

const upload = multer({
  dest: 'uploads/',
  fileFilter: (req, file, cb) => {
    if (ALLOWED_TYPES.includes(file.mimetype)) {
      cb(null, true);
    } else {
      cb(new Error('Invalid file type'));
    }
  },
});

app.post('/api/upload', upload.single('file'), async (req, res) => {
  const type = await fileTypeFromFile(req.file.path);
  if (!type || !ALLOWED_TYPES.includes(type.mime)) {
    await fs.unlink(req.file.path);
    return res.status(400).json({ error: 'Invalid file content' });
  }
  res.json({ filename: req.file.filename });
});
