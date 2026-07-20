// SAFE: Circuit breaker prevents retries when downstream is unhealthy.
import { Request, Response } from 'express';

const circuitState = { failures: 0, lastFailure: 0, open: false };
const THRESHOLD = 5;
const RESET_MS = 30_000;

export async function handleWebhook(req: Request, res: Response): Promise<void> {
  const event = req.body;

  if (circuitState.open) {
    if (Date.now() - circuitState.lastFailure > RESET_MS) {
      circuitState.open = false;
      circuitState.failures = 0;
    } else {
      console.warn('circuit open, skipping downstream call');
      res.json({ received: true, skipped: true });
      return;
    }
  }

  try {
    await callDownstream(event);
    circuitState.failures = 0;
  } catch (err) {
    circuitState.failures++;
    circuitState.lastFailure = Date.now();
    if (circuitState.failures >= THRESHOLD) {
      circuitState.open = true;
    }
    console.error('downstream failed', err);
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
