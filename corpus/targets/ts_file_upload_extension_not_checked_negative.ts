// SAFE: validate extension against allowlist
import multer from 'multer';
import path from 'path';

const ALLOWED_EXTENSIONS = ['.jpg', '.jpeg', '.png', '.gif', '.webp', '.pdf'];

const storage = multer.diskStorage({
  destination: 'uploads/',
  filename: (req, file, cb) => {
    const ext = path.extname(file.originalname).toLowerCase();
    if (!ALLOWED_EXTENSIONS.includes(ext)) {
      return cb(new Error('Extension not allowed'));
    }
    // SAFE: use random name, not original filename
    cb(null, `${Date.now()}-${crypto.randomUUID()}${ext}`);
  },
});
