// SAFE alternative: sanitize and normalize the filename
import multer from 'multer';

function sanitizeFilename(name: string): string {
  // Remove path separators and dangerous chars
  return name
    .replace(/[/\\]/g, '')
    .replace(/\.\./g, '')
    .replace(/[^a-zA-Z0-9._-]/g, '_')
    .slice(0, 255);
}

const storage = multer.diskStorage({
  destination: 'uploads/',
  filename: (req, file, cb) => {
    const safeName = sanitizeFilename(file.originalname);
    cb(null, `${Date.now()}-${safeName}`);
  },
});
