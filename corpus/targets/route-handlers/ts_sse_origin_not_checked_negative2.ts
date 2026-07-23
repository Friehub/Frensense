// SAFE: Same-origin only via Referer check

import { Request, Response } from 'express';

const EXPECTED_HOST = 'api.example.com';

export function streamHandler(req: Request, res: Response) {
  const host = req.headers.host;
  const origin = req.headers.origin;

  if (origin && host && !origin.includes(host)) {
    res.status(403).end();
    return;
  }

  res.writeHead(200, {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    Connection: 'keep-alive',
  });

  const timer = setInterval(() => {
    res.write(`data: ${JSON.stringify({ type: 'update', data: 'sensitive' })}\n\n`);
  }, 2000);

  req.on('close', () => clearInterval(timer));
}
