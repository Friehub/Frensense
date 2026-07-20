// SAFE alternative: tmp-promise with auto cleanup
import { file as tmpFile } from 'tmp-promise';
import fs from 'fs/promises';

async function processUpload(data: Buffer): Promise<Result> {
  const tmp = await tmpFile({ prefix: 'proc-' });
  try {
    await fs.writeFile(tmp.path, data);
    return await doExpensiveProcessing(tmp.path);
  } finally {
    await tmp.cleanup();
  }
}
