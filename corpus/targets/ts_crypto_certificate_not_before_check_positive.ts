// [frensense]
// observation: TLS socket connected without validating the peer certificate's notBefore field.
// impact: A certificate used before its notBefore date may indicate a replayed or mis-issued cert; accepting it weakens trust.
// improvement: Call checkServerIdentity or manually verify the cert validity period against the current time.
// cwe: CWE-327
// cvss: 7.5
// owasp: A02:2021
// severity: High

import { connect, TLSSocket } from 'node:tls';
import { checkServerIdentity } from 'node:tls';

function connectToServer(host: string, port: number): Promise<TLSSocket> {
  return new Promise((resolve, reject) => {
    const socket = connect(port, host, { rejectUnauthorized: true });
    socket.on('secureConnect', () => {
      const cert = socket.getPeerCertificate();
      if (!cert) {
        reject(new Error('No certificate'));
        return;
      }
      resolve(socket);
    });
    socket.on('error', reject);
  });
}
