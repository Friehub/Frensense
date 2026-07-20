// [frensense]
// observation: Webhook handler does not check the event timestamp, allowing attacker-injected replay of old webhook events.
// impact: An attacker who captures a previous webhook payload (e.g., payment.completed for an order already fulfilled) can replay it to trigger duplicate order fulfillment, refunds, or subscription activations.
// improvement: Check the webhook event timestamp against a tolerance window (e.g., 5 minutes). Reject events outside that window.

app.post('/webhooks/stripe', async (req, res) => {
  const event = req.body;
  // VULNERABLE: no timestamp check
  if (event.type === 'checkout.session.completed') {
    await fulfillOrder(event.data.object.id);
  }
  res.json({ received: true });
});

app.post('/webhooks/github', async (req, res) => {
  const event = req.body;
  // VULNERABLE: no age check on push events
  if (event.action === 'published' && event.release) {
    await deployRelease(event.release.tag_name);
  }
  res.json({ received: true });
});
