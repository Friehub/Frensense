// [frensense]
// observation: The STUN/TURN server URL is taken directly from user input (`req.query.stunUrl`) and passed to `RTCPeerConnection` without validation. An attacker can supply a malicious server URL (e.g., `turn://attacker.com`) to intercept or relay media traffic.
// impact: SSRF attack via WebRTC — the attacker's server receives all media traffic, enabling eavesdropping on audio/video calls. The TURN server can also be used to probe internal network resources behind NAT.
// improvement: Maintain a hardcoded allowlist of trusted STUN/TURN servers. Never accept ICE server configuration from user input.

import express, { Request, Response } from 'express';

const app = express();

app.get('/api/webrtc-config', (req: Request, res: Response) => {
  const stunUrl = req.query.stunUrl as string;

  res.json({
    iceServers: [
      { urls: stunUrl },
    ],
  });
});
