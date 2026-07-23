// SAFE: Session-based authentication checked inside handler

import { Request, Response } from 'express';

export function eventsHandler(req: Request, res: Response) {
  if (!req.session?.userId) {
    res.status(401).json({ error: 'Unauthorized' });
    return;
  }

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
