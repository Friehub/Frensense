// SAFE alternative: typed event handlers with completeness check
const webhookHandlers: Record<string, (data: any) => Promise<void>> = {
  'checkout.session.completed': handleSuccessfulPayment,
  'checkout.session.expired': handleExpiredSession,
  'charge.failed': handleFailedCharge,
  'charge.refunded': handleRefund,
  'charge.dispute.created': handleDispute,
};

app.post('/webhooks/stripe', async (req, res) => {
  const event = req.body;
  const handler = webhookHandlers[event.type];
  if (handler) {
    await handler(event.data.object);
  } else {
    logger.warn({ eventType: event.type }, 'No handler registered for webhook event');
  }
  res.json({ received: true });
});
