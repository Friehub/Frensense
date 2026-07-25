// [frensense]
// observation: "Multer disk storage uses the user-provided filename directly, allowing path traversal via '../' sequences in the filename."
// impact: "An attacker can overwrite arbitrary files on the server by providing a filename like '../../../etc/config.json' in the upload form."
// improvement: "Sanitize the filename or use a UUID-based name instead of trusting user input."
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

import multer from 'multer';
import express from 'express';
import path from 'path';

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
