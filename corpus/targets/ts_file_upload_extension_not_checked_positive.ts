// [frensense]
// observation: File upload does not validate the file extension. Users can upload files with any extension, including executable or script types.
// impact: Attackers upload .php, .jsp, .asp, .cgi, or .pl files that the web server executes. Combined with a web-accessible upload directory, this gives remote code execution.
// improvement: Allow only common image/document extensions (.jpg, .png, .pdf) and reject all executable extensions.

import multer from 'multer';

const storage = multer.diskStorage({
  destination: 'uploads/',
  filename: (req, file, cb) => {
    // VULNERABLE: preserves original extension
    cb(null, Date.now() + '-' + file.originalname);
  },
});

const upload = multer({ storage });

app.post('/api/upload', upload.single('file'), (req, res) => {
  res.json({ filename: req.file.filename });
});
