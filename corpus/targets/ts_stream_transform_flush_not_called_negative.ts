// SAFE: _flush implemented to push buffered data before stream ends
import { Transform, TransformCallback } from 'node:stream';

class ChunkAggregator extends Transform {
  private chunks: Buffer[] = [];

  _transform(chunk: Buffer, _encoding: BufferEncoding, callback: TransformCallback): void {
    this.chunks.push(chunk);
    callback();
  }

  _flush(callback: TransformCallback): void {
    this.push(Buffer.concat(this.chunks));
    callback();
  }
}

function createAggregator(): ChunkAggregator {
  return new ChunkAggregator();
}
