// [frensense]
// observation: SHA-1 used for digital signature or certificate verification.
// impact: SHA-1 is vulnerable to SHAttered collision attack (chosen-prefix collision for ~$110K). Accepted CA signatures using SHA-1 were deprecated in 2017.
// improvement: Use SHA-256 or SHA-3 for integrity verification. Use HMAC-SHA256 for authenticated messages.
// cwe: CWE-327
// cvss: 7.5
// owasp: A02:2021
// severity: High

import { createHash, createHmac } from 'node:crypto';

function hashFile(fileBuffer: Buffer): string {
  // VULNERABLE: SHA-1 is deprecated for security uses
  return createHash('sha1').update(fileBuffer).digest('hex');
}

function signData(data: string, key: string): string {
  // VULNERABLE: SHA-1 HMAC is weak
  return createHmac('sha1', key).update(data).digest('hex');
}

function verifyChecksum(storedChecksum: string, data: Buffer): boolean {
  return hashFile(data) === storedChecksum;
}
