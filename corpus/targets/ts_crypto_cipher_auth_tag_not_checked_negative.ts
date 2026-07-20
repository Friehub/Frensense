// SAFE: Auth tag set before finalizing decryption
import { createDecipheriv } from 'node:crypto';

function decrypt(ciphertext: Buffer, key: Buffer, iv: Buffer, authTag: Buffer): string {
  const decipher = createDecipheriv('aes-256-gcm', key, iv);
  decipher.setAuthTag(authTag);
  const plain = decipher.update(ciphertext);
  return plain + decipher.final('utf8');
}
