// SAFE alternative: Check remaining space before write
import { Buffer } from 'node:buffer';

function packInt(buf: Buffer, value: number, offset: number): void {
  if (offset < 0 || offset + 4 > buf.length) {
    throw new RangeError(`Cannot write uint32 at offset ${offset}`);
  }
  buf.writeUInt32BE(value, offset);
}

function writeString(buf: Buffer, data: string, offset: number): number {
  const maxBytes = buf.length - offset;
  if (maxBytes <= 0) return 0;
  return buf.write(data, offset, maxBytes, 'utf8');
}
