// [frensense]
// observation: AES-GCM decryption performed without calling setAuthTag, omitting authentication tag verification.
// impact: An attacker can tamper with the ciphertext undetected; GCM provides no integrity without tag verification.
// improvement: Always call decipher.setAuthTag(tag) before decipher.final() and check that final() does not throw.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import { createDecipheriv } from 'node:crypto';

function decrypt(ciphertext: Buffer, key: Buffer, iv: Buffer): string {
  const decipher = createDecipheriv('aes-256-gcm', key, iv);
  const plain = decipher.update(ciphertext);
  return plain + decipher.final('utf8');
}
