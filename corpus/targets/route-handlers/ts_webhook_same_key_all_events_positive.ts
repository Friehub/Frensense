// [frensense]
// observation: Same webhook signing key reused across all event types — a leaked Stripe webhook secret for `invoice.paid` can forge `checkout.session.completed` events.
// impact: Attacker who acquires the key via one integration path (e.g., repo leak, error log) can impersonate any webhook event type, triggering payments, refunds, or account changes.
// improvement: Derive per-event-type signing keys or verify event type against a allowlist scoped per endpoint.

import { Request, Response } from 'express';
import crypto from 'crypto';

const WEBHOOK_SECRET = process.env.STRIPE_WEBHOOK_SECRET!;

export function handleStripeWebhook(req: Request, res: Response): void {
  const sig = req.headers['stripe-signature'] as string;
  const payload = req.body;

  const expectedSig = crypto
    .createHmac('sha256', WEBHOOK_SECRET)
    .update(JSON.stringify(payload))
    .digest('hex');

  const received = sig.split(',').find(s => s.startsWith('v1='))?.split('=')[1];
  if (expectedSig !== received) {
    res.status(401).json({ error: 'invalid signature' });
    return;
  }

  const event = payload as { type: string; data: { object: any } };

  if (event.type === 'invoice.paid') {
    void fulfillSubscription(event.data.object);
  }
  if (event.type === 'checkout.session.completed') {
    void activateAccount(event.data.object);
  }
  if (event.type === 'customer.subscription.deleted') {
    void deactivateAccount(event.data.object);
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
