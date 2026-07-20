// SAFE alternative: Use checkServerIdentity which validates validity period
import { connect, TLSSocket, checkServerIdentity } from 'node:tls';

function connectToServer(host: string, port: number): Promise<TLSSocket> {
  return new Promise((resolve, reject) => {
    const socket = connect(port, host, {
      rejectUnauthorized: true,
      checkServerIdentity: (hostname, cert) => {
        const err = checkServerIdentity(hostname, cert);
        if (err) return err;
        const now = Date.now();
        if (now < new Date(cert.valid_from).getTime()) {
          return new Error(`Certificate for ${hostname} is not yet valid`);
        }
        return undefined;
      },
    });
    socket.on('secureConnect', () => resolve(socket));
    socket.on('error', reject);
  });
}
