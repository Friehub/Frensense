// SAFE: try/finally guarantees cleanup
import fs from 'fs';
import os from 'os';
import path from 'path';

function processUpload(data: Buffer): Result {
  const tmpPath = path.join(os.tmpdir(), `proc_${Date.now()}.tmp`);
  try {
    fs.writeFileSync(tmpPath, data);
    return doExpensiveProcessing(tmpPath);
  } finally {
    try { fs.unlinkSync(tmpPath); } catch { /* ignore */ }
  }
}

app.post('/api/upload', (req, res) => {
  try {
    const result = processUpload(req.body.file);
    res.json(result);
  } catch (err) {
    res.status(500).json({ error: 'Processing failed' });
  }
});
