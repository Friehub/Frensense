// SAFE alternative: Flush via async generator pattern
import { Readable, Transform, TransformCallback } from 'node:stream';

async function* aggregateChunks(src: AsyncIterable<Buffer>): AsyncGenerator<Buffer> {
  const chunks: Buffer[] = [];
  for await (const chunk of src) {
    chunks.push(chunk);
  }
  yield Buffer.concat(chunks);
}

async function processStream(input: Readable): Promise<Buffer> {
  const results: Buffer[] = [];
  for await (const chunk of aggregateChunks(input)) {
    results.push(chunk);
  }
  return Buffer.concat(results);
}
