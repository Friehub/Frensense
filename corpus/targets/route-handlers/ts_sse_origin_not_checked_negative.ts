// SAFE: Origin header validated against allowlist

import { Request, Response } from 'express';

const ALLOWED_ORIGINS = ['https://app.example.com', 'https://admin.example.com'];

export function streamHandler(req: Request, res: Response) {
  const origin = req.headers.origin;
  if (!origin || !ALLOWED_ORIGINS.includes(origin)) {
    res.status(403).end();
    return;
  }

  res.writeHead(200, {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    Connection: 'keep-alive',
    'Access-Control-Allow-Origin': origin,
  });

  const timer = setInterval(() => {
    res.write(`data: ${JSON.stringify({ type: 'update', data: 'sensitive' })}\n\n`);
  }, 2000);

  req.on('close', () => clearInterval(timer));
}
