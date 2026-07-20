// [frensense]
// observation: Webhook handler processes all incoming events without checking the event type against a whitelist, allowing unexpected event types to trigger unintended logic.
// impact: A Stripe `payment_intent.canceled` event arriving at the `checkout.session.completed` endpoint could trigger account activation or order fulfillment for unpaid/cancelled orders.
// improvement: Validate event type against an explicit allowlist before dispatching to handlers.

import { Request, Response } from 'express';

export async function handleStripeWebhook(req: Request, res: Response): Promise<void> {
  const event = req.body as { type: string; data: { object: any } };
  await handleEvent(event);
  res.json({ received: true });
}

async function handleEvent(event: { type: string; data: { object: any } }): Promise<void> {
  if (event.type === 'checkout.session.completed') {
    await fulfillOrder(event.data.object);
  } else if (event.type === 'invoice.paid') {
    await updateSubscription(event.data.object);
  }
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
