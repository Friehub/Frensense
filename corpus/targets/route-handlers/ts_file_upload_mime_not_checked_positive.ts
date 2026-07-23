// [frensense]
// observation: File upload handler trusts the Content-Type header from the client without server-side MIME validation.
// impact: An attacker can upload a .php or .asp file with Content-Type: image/jpeg. If the file is stored in a web-accessible directory, the attacker can execute arbitrary code on the server.
// improvement: Verify the file's MIME type server-side using magic bytes (file signature) rather than trusting the client-supplied Content-Type.

import multer from 'multer';

const upload = multer({ dest: 'uploads/' });

app.post('/api/upload', upload.single('file'), (req, res) => {
  // VULNERABLE: no MIME type validation
  const file = req.file;
  res.json({ filename: file.filename, mime: file.mimetype });
});
