// [frensense]
// observation: crypto.sign called without specifying a hash algorithm, defaulting to RSA-SHA1.
// impact: RSA-SHA1 is deprecated and vulnerable to collision attacks; an attacker can forge signatures.
// improvement: Explicitly pass a strong hash algorithm like 'sha256' as the second argument to crypto.sign.
// cwe: CWE-327
// cvss: 7.5
// owasp: A02:2021
// severity: High

import { sign, createSign } from 'node:crypto';
import { readFileSync } from 'node:fs';

function signData(data: Buffer, privateKey: string | Buffer): Buffer {
  return sign(null, data, privateKey);
}

function signFile(path: string): Buffer {
  const content = readFileSync(path);
  const privateKey = readFileSync('./key.pem');
  return sign(undefined, content, privateKey);
}
