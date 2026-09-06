// SAFE: User data sanitized and newlines escaped

import { Request, Response } from 'express';

function sanitizeSSEData(input: string): string {
  return input.replace(/\n/g, '\\n').replace(/\r/g, '\\r');
}

export function notifyUser(req: Request, res: Response) {
  const message = req.query.message as string;
  const sanitized = sanitizeSSEData(message);

  res.writeHead(200, {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    Connection: 'keep-alive',
  });

  res.write(`data: ${sanitized}\n\n`);
  res.end();
}
