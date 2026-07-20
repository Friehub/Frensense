// SAFE: Validate notBefore field against current time
import { connect, TLSSocket, checkServerIdentity } from 'node:tls';

function connectToServer(host: string, port: number): Promise<TLSSocket> {
  return new Promise((resolve, reject) => {
    const socket = connect(port, host, { rejectUnauthorized: true });
    socket.on('secureConnect', () => {
      const cert = socket.getPeerCertificate();
      const now = new Date();
      if (!cert || !cert.valid_from || new Date(cert.valid_from) > now) {
        reject(new Error('Certificate not yet valid'));
        return;
      }
      resolve(socket);
    });
    socket.on('error', reject);
  });
}
