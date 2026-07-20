// SAFE alternative: store processed event IDs to prevent replay
const processedEvents = new Set<string>();

app.post('/webhooks/stripe', async (req, res) => {
  const sig = req.headers['stripe-signature'];
  const event = constructEvent(req.body, sig, process.env.STRIPE_WEBHOOK_SECRET);

  // SAFE: idempotency check
  if (processedEvents.has(event.id)) {
    return res.json({ received: true, duplicate: true });
  }
  processedEvents.add(event.id);

  const eventTime = event.created * 1000;
  if (Date.now() - eventTime > 5 * 60 * 1000) {
    return res.status(400).json({ error: 'Event too old' });
  }
  // process event
  res.json({ received: true });
});
