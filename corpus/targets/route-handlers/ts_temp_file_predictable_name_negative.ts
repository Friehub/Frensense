// SAFE: use mkdtemp for unique directory with random suffix
import fs from 'fs/promises';
import os from 'os';
import path from 'path';

async function saveTempFile(userId: string, data: Buffer): Promise<string> {
  const tmpDir = await fs.mkdtemp(path.join(os.tmpdir(), 'upload-'));
  const tempPath = path.join(tmpDir, 'file.bin');
  await fs.writeFile(tempPath, data);
  return tempPath;
}

app.post('/api/process', async (req, res) => {
  const tempFile = await saveTempFile(req.user.id, req.body.fileData);
  const result = processFile(tempFile);
  await fs.unlink(tempFile).catch(() => {});
  res.json(result);
});
