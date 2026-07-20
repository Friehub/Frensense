// SAFE: pipeline called with error callback, resources cleaned
import { createReadStream, createWriteStream } from 'node:fs';
import { pipeline } from 'node:stream';

function copyFile(src: string, dest: string): Promise<void> {
  return new Promise((resolve, reject) => {
    const read = createReadStream(src);
    const write = createWriteStream(dest);
    pipeline(read, write, (err) => {
      if (err) reject(err);
      else resolve();
    });
  });
}
