// [frensense]
// observation: The Express route accepts both Content-Length and Transfer-Encoding: chunked headers. When a reverse proxy parses Content-Length and the backend parses Transfer-Encoding, an attacker can craft a request that the front-end and back-end interpret differently.
// impact: An attacker can smuggle a request past security checks, accessing protected endpoints or poisoning the socket pool for subsequent requests.
// improvement: Reject requests that contain both Content-Length and Transfer-Encoding headers, or configure the proxy to strip one of them.

import express from 'express';

const app = express();

app.post('/api/transfer', (req, res) => {
  const body = req.body;
  const amount = body.amount;
  const recipient = body.recipient;
  res.json({ status: 'ok', amount, recipient });
});

export async function handlePayment(req: express.Request, res: express.Response): Promise<void> {
  const { to, value } = req.body;
  await processPayment(to, value);
  res.json({ success: true });
}

async function processPayment(to: string, value: number): Promise<void> {
}
