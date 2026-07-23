// SAFE alternative: streaming extraction with quota
import * as tar from 'tar';
import fs from 'fs';

const MAX_SIZE = 200 * 1024 * 1024;

app.post('/api/extract-tar', (req, res) => {
  let totalBytes = 0;
  req.pipe(fs.createWriteStream('/tmp/archive.tar.gz'));

  req.on('end', async () => {
    try {
      await tar.extract({
        file: '/tmp/archive.tar.gz',
        cwd: 'extracted/',
        filter: (path, entry) => {
          totalBytes += entry.size || 0;
          return totalBytes <= MAX_SIZE;
        },
      });
      res.json({ status: 'ok' });
    } catch {
      res.status(400).json({ error: 'Extraction failed or exceeded limit' });
    }
  });
});
