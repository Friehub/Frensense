// [frensense]
// observation: Temporary file created but not deleted on error paths. Over time, orphaned temp files accumulate.
// impact: Disk space fills up with unprocessed temp files, leading to 'No space left on device' errors. In cloud environments, this can also increase storage costs and cause deployment failures.
// improvement: Use try/finally or utilities like tmp-promise that auto-cleanup. Delete temp files in both success and error paths.

import fs from 'fs';
import os from 'os';
import path from 'path';

function processUpload(data: Buffer): Result {
  const tmpPath = path.join(os.tmpdir(), `proc_${Date.now()}.tmp`);
  fs.writeFileSync(tmpPath, data);

  const result = doExpensiveProcessing(tmpPath);

  // VULNERABLE: if doExpensiveProcessing throws, temp file persists
  fs.unlinkSync(tmpPath);
  return result;
}

app.post('/api/upload', (req, res) => {
  try {
    const result = processUpload(req.body.file);
    res.json(result);
  } catch (err) {
    // VULNERABLE: processUpload may have left temp files
    res.status(500).json({ error: 'Processing failed' });
  }
});
