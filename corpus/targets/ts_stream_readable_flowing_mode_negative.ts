// SAFE: Attach 'data' listener before resuming, or pipe to writable
import { createReadStream, createWriteStream } from 'node:fs';
import { Readable } from 'node:stream';

function collectStream(stream: Readable): Promise<Buffer[]> {
  return new Promise((resolve) => {
    const chunks: Buffer[] = [];
    stream.on('data', (chunk: Buffer) => chunks.push(chunk));
    stream.on('end', () => resolve(chunks));
    stream.resume();
  });
}

function properCopy(path: string, dest: string): void {
  const src = createReadStream(path);
  const dst = createWriteStream(dest);
  src.pipe(dst);
}
