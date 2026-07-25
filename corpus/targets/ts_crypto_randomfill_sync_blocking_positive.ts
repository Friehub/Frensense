// [frensense]
// observation: crypto.randomFillSync called inside an async request handler, blocking the event loop.
// impact: Synchronous random generation stalls the Node.js event loop, degrading throughput for all concurrent requests.
// improvement: Use the async randomFill or randomBytes variant to yield control back to the event loop.
// cwe: CWE-327
// cvss: 7.5
// owasp: A02:2021
// severity: High

import { randomFillSync } from 'node:crypto';
import { createServer } from 'node:http';

const server = createServer((_req, res) => {
  const buf = Buffer.alloc(64);
  randomFillSync(buf);
  const token = buf.toString('hex');
  res.end(token);
});

server.listen(3000);
