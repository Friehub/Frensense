// SAFE: Default handler rejects unknown event types instead of silently ignoring.
import { Request, Response } from 'express';

export async function handleStripeWebhook(req: Request, res: Response): Promise<void> {
  const event = req.body as { type: string; data: { object: any } };

  switch (event.type) {
    case 'checkout.session.completed':
      await fulfillOrder(event.data.object);
      break;
    case 'invoice.paid':
      await updateSubscription(event.data.object);
      break;
    default:
      res.status(400).json({ error: 'unsupported event type' });
      return;
  }
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
