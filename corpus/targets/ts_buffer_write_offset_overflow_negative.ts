// SAFE: Validate offset before writing to buffer
import { Buffer } from 'node:buffer';

function writeToBuffer(buf: Buffer, data: string, offset: number): number {
  const byteLen = Buffer.byteLength(data, 'utf8');
  if (offset < 0 || offset + byteLen > buf.length) {
    throw new RangeError('Offset exceeds buffer capacity');
  }
  return buf.write(data, offset, 'utf8');
}

function serializeMessage(buf: Buffer, fields: Array<{ value: string; pos: number }>): void {
  for (const f of fields) {
    const byteLen = Buffer.byteLength(f.value, 'utf8');
    if (f.pos < 0 || f.pos + byteLen > buf.length) {
      throw new RangeError('Field position out of bounds');
    }
    buf.write(f.value, f.pos, 'utf8');
  }
}
