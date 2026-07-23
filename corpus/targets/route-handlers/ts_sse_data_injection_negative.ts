// SAFE: User data JSON-encoded before writing to SSE stream

import { Request, Response } from 'express';

export function notifyUser(req: Request, res: Response) {
  const message = req.query.message as string;

  res.writeHead(200, {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    Connection: 'keep-alive',
  });

  res.write(`data: ${JSON.stringify({ text: message })}\n\n`);
  res.end();
}
