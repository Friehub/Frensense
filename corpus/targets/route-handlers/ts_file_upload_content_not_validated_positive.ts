// [frensense]
// observation: File content is not inspected or validated server-side. An image upload can accept a renamed executable, script, or polyglot file.
// impact: Even if MIME type and extension are checked, a file with .jpg extension but containing arbitrary binary data passes through. Attackers can upload polyglot files that are valid images AND valid scripts.
// improvement: Inspect file content using library functions (sharp for images, pdf-parse for PDFs) to ensure the file actually contains the expected data type.

import multer from 'multer';

const upload = multer({ dest: 'uploads/', fileFilter: (req, file, cb) => {
  // VULNERABLE: trusts client-reported MIME type
  cb(null, file.mimetype === 'image/jpeg');
}});

app.post('/api/upload', upload.single('file'), (req, res) => {
  res.json({ filename: req.file.filename });
});
