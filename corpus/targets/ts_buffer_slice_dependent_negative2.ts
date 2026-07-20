// SAFE alternative: Use Buffer.copyBytesFrom (Node 20+) for independent copy
import { Buffer } from 'node:buffer';

function parsePacket(packet: Buffer): { header: Buffer; body: Buffer } {
  const header = Buffer.copyBytesFrom(packet, 0, 8);
  const body = Buffer.copyBytesFrom(packet, 8);
  return { header, body };
}
