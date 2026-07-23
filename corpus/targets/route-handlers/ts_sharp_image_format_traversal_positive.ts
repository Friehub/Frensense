// [frensense]
// observation: "File extension is not validated, and the output path is derived from the user-controlled filename, enabling path traversal via the file extension."
// impact: "An attacker can upload a file with a crafted name like '.../.../etc/passwd.png' or '../../../etc/cronjob' to write files outside the intended directory."
// improvement: "Validate the file extension against an allowlist, sanitize the filename, and do not use user input in file paths."

import sharp from 'sharp';
import express from 'express';
import multer from 'multer';
import path from 'path';

const app = express();
const upload = multer({ dest: '/tmp/uploads/' });

app.post('/upload', upload.single('image'), async (req, res) => {
  const outputPath = path.join('/uploads', req.file.originalname);

  await sharp(req.file.path)
    .resize(800, 600)
    .jpeg()
    .toFile(outputPath);

  res.json({ url: `/uploads/${req.file.originalname}` });
});
