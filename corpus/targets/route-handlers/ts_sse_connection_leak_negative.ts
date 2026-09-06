// SAFE: Connection cleaned up on client disconnect

import { Request, Response } from 'express';

const clients: Map<number, Response> = new Map();
let nextId = 0;

export function addClient(req: Request, res: Response) {
  res.writeHead(200, {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    Connection: 'keep-alive',
  });

  const id = nextId++;
  clients.set(id, res);

  const interval = setInterval(() => {
    res.write(`data: ${JSON.stringify({ message: 'heartbeat' })}\n\n`);
  }, 5000);

  req.on('close', () => {
    clearInterval(interval);
    clients.delete(id);
  });
}
