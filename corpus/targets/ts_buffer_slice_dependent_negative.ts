// SAFE: Create independent copy via Buffer.from(slice) after slicing
import { Buffer } from 'node:buffer';

function parsePacket(packet: Buffer): { header: Buffer; body: Buffer } {
  const header = Buffer.from(packet.subarray(0, 8));
  const body = Buffer.from(packet.subarray(8));
  return { header, body };
}

function processMessage(msg: Buffer): Buffer {
  const id = Buffer.from(msg.subarray(0, 4));
  id.writeUInt32BE(0, 0);
  return id;
}
