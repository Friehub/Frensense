// SAFE alternative: use res.sendFile for individual files
import express from 'express';
import { join } from 'path';

const app = express();

app.get('/files/:filename', (req, res) => {
  const safeName = req.params.filename.replace(/[/\\]/g, '');
  const filePath = join(__dirname, '../files', safeName);
  if (!filePath.startsWith(join(__dirname, '../files'))) {
    return res.status(403).json({ error: 'Forbidden' });
  }
  res.sendFile(filePath);
});
