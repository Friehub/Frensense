// [frensense]
// observation: readable.resume() called without a 'data' event listener, putting the stream into flowing mode with no consumer.
// impact: Data is silently discarded while the stream remains open, wasting I/O resources and potentially causing backpressure issues upstream.
// improvement: Either attach a 'data' listener before resume() or use pipe() to direct the data to a writable.

import { createReadStream } from 'node:fs';
import { Readable } from 'node:stream';

function drainStream(stream: Readable): void {
  stream.resume();
}

function autoFlush(path: string): void {
  const src = createReadStream(path);
  src.resume();
}
