// SAFE: Use a hardcoded allowlist of trusted STUN/TURN servers.

import express, { Request, Response } from 'express';

const TRUSTED_ICE_SERVERS = [
  { urls: 'stun:stun.l.google.com:19302' },
  { urls: 'stun:stun1.l.google.com:19302' },
];

const app = express();

app.get('/api/webrtc-config', (_req: Request, res: Response) => {
  res.json({ iceServers: TRUSTED_ICE_SERVERS });
});
