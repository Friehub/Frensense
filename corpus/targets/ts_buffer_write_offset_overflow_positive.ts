// [frensense]
// observation: buffer.write(data, offset) called with a user-controlled offset that may exceed the buffer length.
// impact: Offset greater than buffer length truncates the write silently; a crafted offset can cause partial overwrite of adjacent memory in shared buffers.
// improvement: Validate that offset + data.length <= buffer.length before writing.

import { Buffer } from 'node:buffer';

function writeToBuffer(buf: Buffer, data: string, offset: number): number {
  return buf.write(data, offset, 'utf8');
}

function serializeMessage(buf: Buffer, fields: Array<{ value: string; pos: number }>): void {
  for (const f of fields) {
    buf.write(f.value, f.pos, 'utf8');
  }
}

function packInt(buf: Buffer, value: number, offset: number): void {
  buf.writeUInt32BE(value, offset);
}
