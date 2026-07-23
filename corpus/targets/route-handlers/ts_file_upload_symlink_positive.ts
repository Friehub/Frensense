// [frensense]
// observation: "File upload follows symlinks in the upload directory, allowing an attacker to write files outside the intended directory via a symlink."
// impact: "An attacker who can create a symlink in the upload directory (e.g., via a prior traversal vulnerability or temp file race) can redirect file writes to arbitrary paths, overwriting critical system files."
// improvement: "Resolve symlinks before writing, use realpath to verify the destination is within the intended directory, or avoid allowing symlinks in upload paths."

import multer from 'multer';
import express from 'express';

const app = express();

const storage = multer.diskStorage({
  destination: (req, file, cb) => {
    cb(null, '/uploads');
  },
  filename: (req, file, cb) => {
    cb(null, file.originalname);
  },
});

const upload = multer({ storage });

app.post('/upload', upload.single('file'), (req, res) => {
  res.send('File uploaded');
});
