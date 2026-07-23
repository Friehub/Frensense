// SAFE: limit decompressed size and entry count
import AdmZip from 'adm-zip';
import path from 'path';
import fs from 'fs/promises';

const MAX_SIZE = 100 * 1024 * 1024; // 100 MB
const MAX_ENTRIES = 1000;

app.post('/api/extract-zip', async (req, res) => {
  const zip = new AdmZip(req.file.path);
  const entries = zip.getEntries();

  if (entries.length > MAX_ENTRIES) {
    return res.status(400).json({ error: 'Too many entries' });
  }

  let totalSize = 0;
  for (const entry of entries) {
    if (entry.isDirectory) continue;
    const size = entry.getData().length;
    totalSize += size;
    if (totalSize > MAX_SIZE) {
      return res.status(400).json({ error: 'Archive too large' });
    }
  }

  zip.extractAllTo('extracted/', true);
  res.json({ status: 'ok' });
});
