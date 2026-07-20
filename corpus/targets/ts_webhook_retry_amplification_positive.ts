// [frensense]
// observation: Webhook handler retries downstream API calls on failure without circuit breaker or backoff, causing retry storms that amplify load.
// impact: A downstream outage triggers exponential retry amplification, DDoSing the downstream service and incurring excessive costs.
// improvement: Implement exponential backoff, circuit breaker, and max retry limits.

import { Request, Response } from 'express';

export async function handleWebhook(req: Request, res: Response): Promise<void> {
  const event = req.body;
  try {
    await callDownstream(event);
  } catch (err) {
    for (let i = 0; i < 5; i++) {
      try {
        await callDownstream(event);
        break;
      } catch {}
    }
  }
  res.json({ received: true });
}

async function callDownstream(event: any): Promise<void> {
  const resp = await fetch('https://api.internal.example.com/process', {
    method: 'POST',
    body: JSON.stringify(event),
    headers: { 'Content-Type': 'application/json' },
  });
  if (!resp.ok) {
    throw new Error(`downstream returned ${resp.status}`);
  }
}
