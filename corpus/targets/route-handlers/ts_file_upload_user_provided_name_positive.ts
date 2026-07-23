// [frensense]
// observation: File is stored using the user-provided filename, allowing path traversal via '../' in the filename.
// impact: An attacker can upload a file named '../../../etc/cron.d/malicious' to write to arbitrary locations on the filesystem. This can overwrite system files, create cron jobs, or modify application code.
// improvement: Never use user-provided filenames. Generate a safe filename using UUID or hash, and store the original name separately in a database.

import multer from 'multer';

const storage = multer.diskStorage({
  destination: 'uploads/',
  filename: (req, file, cb) => {
    // VULNERABLE: user-provided filename — path traversal possible
    cb(null, file.originalname);
  },
});

const upload = multer({ storage });

app.post('/api/upload', upload.single('file'), (req, res) => {
  res.json({ filename: req.file.filename });
});
