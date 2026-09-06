// [frensense]
// observation: 500 error page displays internal IP address, hostname, or container ID in the HTML response or JSON body, exposing network topology.
// impact: Internal network reconnaissance — an attacker learns server IPs (e.g., 10.0.1.5, 172.17.0.2) and hostnames, enabling targeted SSRF attacks and lateral movement planning.
// improvement: Never include system metadata in error responses. Log internally and return a generic message.
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium

import { Request, Response, NextFunction } from 'express';
import os from 'os';

export function errorHandler(err: Error, req: Request, res: Response, _next: NextFunction): void {
  res.status(500).send(`
    <html>
      <body>
        <h1>Internal Server Error</h1>
        <p>Something went wrong on our end.</p>
        <hr>
        <pre>
Host: ${os.hostname()}
IP: ${Object.values(os.networkInterfaces()).flat().filter(i => i && !i.internal).map(i => i?.address).join(', ')}
Server Time: ${new Date().toISOString()}
Error: ${err.message}
        </pre>
      </body>
    </html>
  `);
}
