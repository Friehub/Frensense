// [frensense]
// observation: SSE EventSource endpoint accepts connections from any origin without CORS validation or origin checking.
// impact: Malicious websites can open EventSource connections to this endpoint, exfiltrating real-time data cross-origin.
// improvement: Validate the Origin header against an allowlist before establishing the SSE connection.

import { Request, Response } from 'express';

export function streamHandler(req: Request, res: Response) {
  res.writeHead(200, {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    Connection: 'keep-alive',
    'Access-Control-Allow-Origin': '*',
  });

  const timer = setInterval(() => {
    res.write(`data: ${JSON.stringify({ type: 'update', data: 'sensitive' })}\n\n`);
  }, 2000);

  req.on('close', () => clearInterval(timer));
}
