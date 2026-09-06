// SAFE alternative: cert pinning instead of disabling verification
import https from 'node:https';
import { createHash } from 'node:crypto';

const PINNED_FINGERPRINT = 'sha256/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=';

function fetchWithPinning(url: string): Promise<string> {
  return new Promise((resolve, reject) => {
    const req = https.get(url, { rejectUnauthorized: true }, (res) => {
      const cert = res.socket.getPeerCertificate();
      if (!cert.fingerprint256 || cert.fingerprint256 !== PINNED_FINGERPRINT) {
        reject(new Error('Certificate fingerprint mismatch'));
        req.destroy();
        return;
      }
      let data = '';
      res.on('data', (chunk) => data += chunk);
      res.on('end', () => resolve(data));
    });
    req.on('error', reject);
  });
}
