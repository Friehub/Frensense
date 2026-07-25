// [frensense]
// observation: SSE endpoint is exposed without authentication middleware, allowing any unauthenticated client to connect and receive events.
// impact: Unauthenticated users can listen to server-sent events, potentially leaking sensitive real-time data.
// improvement: Apply authentication middleware to the SSE route before establishing the connection.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import { Request, Response } from 'express';

export function eventsHandler(req: Request, res: Response) {
  res.writeHead(200, {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    Connection: 'keep-alive',
  });

  const interval = setInterval(() => {
    res.write(`data: ${JSON.stringify({ timestamp: Date.now() })}\n\n`);
  }, 1000);

  req.on('close', () => clearInterval(interval));
}
