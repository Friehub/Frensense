// SAFE alternative: Catch authentication failure from final()
import { createDecipheriv } from 'node:crypto';

function decrypt(ciphertext: Buffer, key: Buffer, iv: Buffer, authTag: Buffer): string {
  const decipher = createDecipheriv('aes-256-gcm', key, iv);
  decipher.setAuthTag(authTag);
  const plain = decipher.update(ciphertext);
  try {
    return plain + decipher.final('utf8');
  } catch {
    throw new Error('Authentication failed: ciphertext may be tampered');
  }
}
