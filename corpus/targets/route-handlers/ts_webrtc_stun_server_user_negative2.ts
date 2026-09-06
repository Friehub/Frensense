// SAFE: Validate user-supplied STUN URL against an allowlist before using it.

import express, { Request, Response } from 'express';

const ALLOWED_STUN_DOMAINS = new Set([
  'stun.l.google.com',
  'stun1.l.google.com',
  'stun.twilio.com',
]);

function isValidStunUrl(url: string): boolean {
  try {
    const parsed = new URL(url);
    return (parsed.protocol === 'stun:' || parsed.protocol === 'turn:' || parsed.protocol === 'turns:')
      && ALLOWED_STUN_DOMAINS.has(parsed.hostname);
  } catch {
    return false;
  }
}

const app = express();

app.get('/api/webrtc-config', (req: Request, res: Response) => {
  const userUrl = req.query.stunUrl as string | undefined;
  const iceServers = userUrl && isValidStunUrl(userUrl)
    ? [{ urls: userUrl }]
    : [{ urls: 'stun:stun.l.google.com:19302' }];

  res.json({ iceServers });
});
