// SAFE: Per-endpoint webhook secrets via environment variable per event type.
import { Request, Response } from 'express';
import crypto from 'crypto';

const WEBHOOK_SECRETS: Record<string, string> = {
  'invoice.paid': process.env.STRIPE_INVOICE_SECRET!,
  'checkout.session.completed': process.env.STRIPE_CHECKOUT_SECRET!,
  'customer.subscription.deleted': process.env.STRIPE_SUBSCRIPTION_SECRET!,
};

export function handleStripeWebhook(req: Request, res: Response): void {
  const sig = req.headers['stripe-signature'] as string;
  const payload = req.body;
  const event = payload as { type: string };
  const eventType = event.type;

  const secret = WEBHOOK_SECRETS[eventType];
  if (!secret) {
    res.status(400).json({ error: 'unknown event type' });
    return;
  }

  const expectedSig = crypto
    .createHmac('sha256', secret)
    .update(JSON.stringify(payload))
    .digest('hex');

  const received = sig.split(',').find(s => s.startsWith('v1='))?.split('=')[1];
  if (expectedSig !== received) {
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
