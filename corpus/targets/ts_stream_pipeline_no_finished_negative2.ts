// SAFE alternative: Use pipeline from stream/promises
import { createReadStream, createWriteStream } from 'node:fs';
import { pipeline } from 'node:stream/promises';

async function copyFile(src: string, dest: string): Promise<void> {
  const read = createReadStream(src);
  const write = createWriteStream(dest);
  await pipeline(read, write);
}
