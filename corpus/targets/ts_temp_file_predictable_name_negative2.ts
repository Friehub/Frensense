// SAFE alternative: use tmp-promise library
import { file as tmpFile } from 'tmp-promise';
import fs from 'fs/promises';

async function saveTempFile(data: Buffer): Promise<{ path: string; cleanup: () => Promise<void> }> {
  const tmp = await tmpFile({ prefix: 'upload-', postfix: '.bin' });
  await fs.writeFile(tmp.path, data);
  return { path: tmp.path, cleanup: tmp.cleanup };
}
