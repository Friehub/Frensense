// SAFE: enforce size limit
import multer from 'multer';

const MAX_SIZE = 10 * 1024 * 1024; // 10 MB

const upload = multer({
  dest: 'uploads/',
  limits: { fileSize: MAX_SIZE },
});

app.post('/api/upload', upload.single('file'), (req, res) => {
  res.json({ size: req.file.size });
});
