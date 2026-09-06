// SAFE: Explicit SHA-256 hash algorithm passed to sign
import { sign } from 'node:crypto';
import { readFileSync } from 'node:fs';

function signData(data: Buffer, privateKey: string | Buffer): Buffer {
  return sign('sha256', data, privateKey);
}

function signFile(path: string): Buffer {
  const content = readFileSync(path);
  const privateKey = readFileSync('./key.pem');
  return sign('sha256', content, privateKey);
}
