// SAFE alternative: Use pipeline or WritableStream with backpressure handling
import { Writable } from 'node:stream';

function createBackpressureWriter(dest: Writable): (data: Buffer) => Promise<void> {
  return function write(data: Buffer): Promise<void> {
    return new Promise((resolve, reject) => {
      const ok = dest.write(data, (err?: Error | null) => {
        if (err) reject(err);
      });
      if (!ok) {
        dest.once('drain', resolve);
      } else {
        resolve();
      }
    });
  };
}
