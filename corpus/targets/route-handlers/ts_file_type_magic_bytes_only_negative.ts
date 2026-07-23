// SAFE: File type is validated by checking magic bytes before accepting the upload

import multer from 'multer';
import express from 'express';
import path from 'path';
import fs from 'fs/promises';

const MAGIC_BYTES: Record<string, Uint8Array> = {
  jpg: new Uint8Array([0xFF, 0xD8, 0xFF]),
  png: new Uint8Array([0x89, 0x50, 0x4E, 0x47]),
  gif: new Uint8Array([0x47, 0x49, 0x46]),
};

async function isValidFileType(filePath: string, ext: string): Promise<boolean> {
  const fd = await fs.open(filePath, 'r');
  const buffer = new Uint8Array(4);
  await fd.read(buffer, 0, 4, 0);
  await fd.close();

  const magic = MAGIC_BYTES[ext];
  if (!magic) return false;
  return buffer.slice(0, magic.length).every((b, i) => b === magic[i]);
}

const storage = multer.diskStorage({
  destination: (req, file, cb) => cb(null, '/tmp'),
  filename: (req, file, cb) => cb(null, `${Date.now()}_${file.originalname}`),
});

const upload = multer({ storage });

app.post('/upload', upload.single('image'), async (req, res) => {
  const ext = path.extname(req.file.originalname).toLowerCase().replace('.', '');
  if (!(await isValidFileType(req.file.path, ext))) {
    await fs.unlink(req.file.path);
    return res.status(400).send('Invalid file content');
  }

  await fs.rename(req.file.path, `/uploads/${req.file.filename}`);
  res.send('File uploaded');
});
