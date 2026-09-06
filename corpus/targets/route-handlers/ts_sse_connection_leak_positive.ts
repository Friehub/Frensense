// [frensense]
// observation: SSE connection is established but never cleaned up when the client disconnects, causing a resource leak.
// impact: Disconnected clients accumulate as open connections, exhausting server resources and causing denial of service.
// improvement: Listen for the 'close' event on the request and clean up timers, intervals, and response references.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { Request, Response } from 'express';

const clients: Response[] = [];

export function addClient(req: Request, res: Response) {
  res.writeHead(200, {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    Connection: 'keep-alive',
  });

  clients.push(res);

  const interval = setInterval(() => {
    res.write(`data: ${JSON.stringify({ message: 'heartbeat' })}\n\n`);
  }, 5000);
}
