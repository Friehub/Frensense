// [frensense]
// observation: File upload endpoint does not enforce a size limit, allowing attackers to upload arbitrarily large files.
// impact: An attacker can upload multi-gigabyte files to exhaust disk space, cause denial of service, or fill up the filesystem partition. In cloud environments with auto-scaling, this can trigger significant cost.
// improvement: Set a reasonable upload size limit using multer limits or a reverse proxy. Validate Content-Length header before processing.

import multer from 'multer';

// VULNERABLE: no size limit
const upload = multer({ dest: 'uploads/' });

app.post('/api/upload', upload.single('file'), (req, res) => {
  res.json({ size: req.file.size });
});

// VULNERABLE: raw body parsing with no limit
app.post('/api/upload-raw', (req, res) => {
  let data = Buffer.alloc(0);
  req.on('data', chunk => data = Buffer.concat([data, chunk]));
  req.on('end', () => {
    fs.writeFileSync('uploads/raw.dat', data);
    res.json({ size: data.length });
  });
});
