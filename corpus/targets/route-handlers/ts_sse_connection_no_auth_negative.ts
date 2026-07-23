// SAFE: Authentication middleware applied before SSE handler

import { Request, Response } from 'express';

function authenticate(req: Request, res: Response, next: () => void) {
  if (!req.headers.authorization) {
    res.status(401).end();
    return;
  }
  next();
}

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

export function registerRoutes(app: any) {
  app.get('/events', authenticate, eventsHandler);
}
