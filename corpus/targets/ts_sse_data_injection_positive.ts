// [frensense]
// observation: User-controlled data is pushed via SSE without encoding or sanitization, allowing content injection into the event stream.
// impact: An attacker can inject malicious data that breaks the SSE protocol, potentially leading to XSS or data stream poisoning.
// improvement: Always JSON-encode user data and avoid embedding raw user input directly into the SSE data field.

import { Request, Response } from 'express';

export function notifyUser(req: Request, res: Response) {
  const userId = req.params.userId;
  const message = req.query.message as string;

  res.writeHead(200, {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    Connection: 'keep-alive',
  });

  res.write(`data: ${message}\n\n`);
  res.end();
}
