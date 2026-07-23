// [frensense]
// observation: Temporary files created with predictable names (/tmp/upload_${userId}.tmp), enabling race condition or symlink attacks.
// impact: An attacker can create a symlink with the predicted temp filename pointing to a sensitive file (e.g., /etc/passwd). When the application writes to the temp file, it overwrites the symlink target.
// improvement: Use mkdtemp or mkstemp-style functions for safe temp file creation with randomized names.

import fs from 'fs';
import os from 'os';
import path from 'path';

function saveTempFile(userId: string, data: Buffer): string {
  // VULNERABLE: predictable temp file name
  const tempPath = path.join(os.tmpdir(), `upload_${userId}.tmp`);
  fs.writeFileSync(tempPath, data);
  return tempPath;
}

app.post('/api/process', (req, res) => {
  const tempFile = saveTempFile(req.user.id, req.body.fileData);
  const result = processFile(tempFile);
  fs.unlinkSync(tempFile);
  res.json(result);
});
