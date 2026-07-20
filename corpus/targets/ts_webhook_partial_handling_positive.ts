// [frensense]
// observation: Webhook handler only processes the success event path, ignoring failed, cancelled, or other status events.
// impact: Duplicate from event-driven missing side effects. Similar to ts_missing_webhook_event_handling, but specifically webhook context.

app.post('/webhooks/stripe', async (req, res) => {
  const event = req.body;
  // VULNERABLE: only handles completed, ignores failed/expired/refunded
  if (event.type === 'checkout.session.completed') {
    await fulfillOrder(event.data.object.id);
  }
  res.json({ received: true });
});
