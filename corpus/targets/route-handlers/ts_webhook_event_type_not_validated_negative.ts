// SAFE: Event type validated against explicit allowlist before processing.
import { Request, Response } from 'express';

type EventHandler = (data: any) => Promise<void>;

const HANDLERS: Record<string, EventHandler> = {
  'checkout.session.completed': fulfillOrder,
  'invoice.paid': updateSubscription,
};

const ALLOWED_EVENTS = new Set(Object.keys(HANDLERS));

export async function handleStripeWebhook(req: Request, res: Response): Promise<void> {
  const event = req.body as { type: string; data: { object: any } };

  if (!ALLOWED_EVENTS.has(event.type)) {
    res.status(400).json({ error: `unexpected event type: ${event.type}` });
    return;
  }

  const handler = HANDLERS[event.type];
  await handler(event.data.object);
  res.json({ received: true });
}

async function fulfillOrder(session: any): Promise<void> {
  await fetch('https://api.example.com/orders', {
    method: 'POST',
    body: JSON.stringify({ sessionId: session.id }),
    headers: { 'Content-Type': 'application/json' },
  });
}

async function updateSubscription(invoice: any): Promise<void> {
  await fetch('https://api.example.com/subscriptions', {
    method: 'PATCH',
    body: JSON.stringify({ invoiceId: invoice.id }),
    headers: { 'Content-Type': 'application/json' },
  });
}
