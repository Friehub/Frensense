// [frensense]
// observation: A Transform stream is implemented without overriding the _flush method, so buffered data is never emitted when the stream ends.
// impact: The final chunk of transformed data is lost; consumers receive an incomplete output.
// improvement: Implement _flush to emit any remaining buffered data before the stream closes.

import { Transform, TransformCallback } from 'node:stream';

class ChunkAggregator extends Transform {
  private chunks: Buffer[] = [];

  _transform(chunk: Buffer, _encoding: BufferEncoding, callback: TransformCallback): void {
    this.chunks.push(chunk);
    callback();
  }
}

function createAggregator(): ChunkAggregator {
  return new ChunkAggregator();
}
