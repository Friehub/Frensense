// SAFE alternative: use typed event handler registry
type WebhookHandler = (data: any) => Promise<void>;
const handlers: Partial<Record<string, WebhookHandler>> = {
  'checkout.session.completed': fulfillOrder,
  'checkout.session.expired': handleExpiredSession,
  'charge.failed': handleFailedCharge,
  'charge.refunded': updateRefundStatus,
};

app.post('/webhooks/stripe', async (req, res) => {
  const event = req.body;
  const handler = handlers[event.type];
  if (handler) {
    await handler(event.data.object);
  } else {
    logger.warn({ type: event.type }, 'Unhandled webhook event');
  }
  res.json({ received: true });
});
