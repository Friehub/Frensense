// SAFE: Using a Set with cleanup via AbortController

import { Request, Response } from 'express';

const connections = new Set<{ res: Response; controller: AbortController }>();

export function addClient(req: Request, res: Response) {
  res.writeHead(200, {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    Connection: 'keep-alive',
  });

  const controller = new AbortController();
  const entry = { res, controller };
  connections.add(entry);

  const interval = setInterval(() => {
    res.write(`data: ${JSON.stringify({ message: 'heartbeat' })}\n\n`);
  }, 5000);

  req.on('close', () => {
    clearInterval(interval);
    connections.delete(entry);
    res.end();
  });
}
