// SAFE alternative: Use createSign with explicit hash algorithm
import { createSign } from 'node:crypto';
import { readFileSync } from 'node:fs';

function signData(data: Buffer, privateKey: string | Buffer): Buffer {
  const signer = createSign('sha256');
  signer.update(data);
  return signer.sign(privateKey);
}

function signFile(path: string): Buffer {
  const content = readFileSync(path);
  const privateKey = readFileSync('./key.pem');
  const signer = createSign('sha384');
  signer.update(content);
  return signer.sign(privateKey);
}
