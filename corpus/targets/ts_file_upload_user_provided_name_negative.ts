// SAFE: generate random filename, store original in database
import multer from 'multer';
import { randomUUID } from 'node:crypto';
import path from 'path';

const storage = multer.diskStorage({
  destination: 'uploads/',
  filename: (req, file, cb) => {
    const ext = path.extname(file.originalname).toLowerCase();
    const safe = ['.jpg', '.png', '.pdf'].includes(ext) ? ext : '.bin';
    // SAFE: random UUID filename prevents traversal and collision
    cb(null, `${randomUUID()}${safe}`);
  },
});

const upload = multer({ storage });
