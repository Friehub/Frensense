// SAFE: Single secret but event-type allowlist prevents cross-event forging.
import { Request, Response } from 'express';
import crypto from 'crypto';

const WEBHOOK_SECRET = process.env.STRIPE_WEBHOOK_SECRET!;

const ALLOWED_EVENTS = new Set([
  'invoice.paid',
  'checkout.session.completed',
  'customer.subscription.deleted',
]);

export function handleStripeWebhook(req: Request, res: Response): void {
  const sig = req.headers['stripe-signature'] as string;
  const payload = req.body;
  const eventType = payload.type as string;

  if (!ALLOWED_EVENTS.has(eventType)) {
    res.status(400).json({ error: 'event type not allowed for this endpoint' });
    return;
  }

  const expectedSig = crypto
    .createHmac('sha256', WEBHOOK_SECRET)
    .update(JSON.stringify(payload))
    .digest('hex');

  const received = sig?.split(',').find(s => s.startsWith('v1='))?.split('=')[1];
  if (!received || !crypto.timingSafeEqual(Buffer.from(expectedSig), Buffer.from(received))) {
    res.status(401).json({ error: 'invalid signature' });
    return;
  }

  if (eventType === 'invoice.paid') {
    void fulfillSubscription(payload.data.object);
  } else if (eventType === 'checkout.session.completed') {
    void activateAccount(payload.data.object);
  } else if (eventType === 'customer.subscription.deleted') {
    void deactivateAccount(payload.data.object);
  }
  res.json({ received: true });
}

async function fulfillSubscription(invoice: any): Promise<void> {
  console.log('fulfill', invoice.id);
}
async function activateAccount(session: any): Promise<void> {
  console.log('activate', session.id);
}
async function deactivateAccount(sub: any): Promise<void> {
  console.log('deactivate', sub.id);
}
