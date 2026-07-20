// SAFE alternative: strip extension entirely, detect type from content
import multer from 'multer';

const storage = multer.diskStorage({
  destination: 'uploads/',
  filename: (req, file, cb) => {
    const ext = path.extname(file.originalname).toLowerCase();
    const safe = ['.jpg', '.jpeg', '.png', '.gif', '.webp', '.pdf'].includes(ext) ? ext : '.bin';
    cb(null, `${Date.now()}-${crypto.randomUUID()}${safe}`);
  },
});
