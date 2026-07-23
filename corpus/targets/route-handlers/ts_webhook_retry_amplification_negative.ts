// SAFE: Exponential backoff and max retry limit prevents amplification.
import { Request, Response } from 'express';

const MAX_RETRIES = 3;
const BASE_DELAY_MS = 1000;

export async function handleWebhook(req: Request, res: Response): Promise<void> {
  const event = req.body;
  try {
    await callDownstreamWithRetry(event);
  } catch (err) {
    console.error('downstream failed after retries', err);
  }
  res.json({ received: true });
}

async function callDownstreamWithRetry(event: any, attempt = 0): Promise<void> {
  try {
    const resp = await fetch('https://api.internal.example.com/process', {
      method: 'POST',
      body: JSON.stringify(event),
      headers: { 'Content-Type': 'application/json' },
    });
    if (!resp.ok) {
      throw new Error(`downstream returned ${resp.status}`);
    }
  } catch (err) {
    if (attempt < MAX_RETRIES) {
      const delay = BASE_DELAY_MS * Math.pow(2, attempt);
      await new Promise(r => setTimeout(r, delay));
      return callDownstreamWithRetry(event, attempt + 1);
    }
    throw err;
  }
}
