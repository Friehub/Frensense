// SAFE: Respect backpressure by waiting for 'drain' when write() returns false
import { Writable } from 'node:stream';

async function writeAll(dest: Writable, data: Buffer): Promise<void> {
  if (!dest.write(data)) {
    await new Promise((resolve) => dest.once('drain', resolve));
  }
}

async function writeChunks(dest: Writable, chunks: Buffer[]): Promise<void> {
  for (const chunk of chunks) {
    if (!dest.write(chunk)) {
      await new Promise((resolve) => dest.once('drain', resolve));
    }
  }
}
