// [frensense]
// observation: buffer.slice() called and the result is used after the original buffer is mutated; slice shares the same underlying memory.
// impact: Subsequent writes to the original buffer silently change the sliced view, causing data corruption and unpredictable behavior.
// improvement: Use buffer.subarray() with explicit documentation, or call Buffer.from(slice) to create an independent copy.

import { Buffer } from 'node:buffer';

function parsePacket(packet: Buffer): { header: Buffer; body: Buffer } {
  const header = packet.slice(0, 8);
  const body = packet.slice(8);
  return { header, body };
}

function processMessage(msg: Buffer): Buffer {
  const id = msg.slice(0, 4);
  id.writeUInt32BE(0, 0);
  return id;
}
