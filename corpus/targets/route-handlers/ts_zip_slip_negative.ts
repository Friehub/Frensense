// SAFE: validate resolved path stays within target directory
import AdmZip from 'adm-zip';
import path from 'path';
import fs from 'fs';

app.post('/api/extract', async (req, res) => {
  const zip = new AdmZip(req.file.path);
  const base = path.resolve('extracted');

  for (const entry of zip.getEntries()) {
    const resolved = path.resolve(base, entry.entryName);
    // SAFE: check both prefix and path traversal with path.relative
    if (!resolved.startsWith(base + path.sep) && resolved !== base) {
      return res.status(400).json({ error: 'Invalid archive entry' });
    }
    if (!entry.isDirectory) {
      const data = entry.getData();
      const dir = path.dirname(resolved);
      await fs.promises.mkdir(dir, { recursive: true });
      await fs.promises.writeFile(resolved, data);
    }
  }

  res.json({ status: 'ok' });
});
