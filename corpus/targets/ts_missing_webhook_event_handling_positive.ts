// [frensense]
// observation: Webhook handler only processes the 'success' or 'completed' event type, ignoring all others (failure, cancelled, expired).
// impact: If the webhook reports a payment failure, subscription expiry, or chargeback, the handler silently ignores it. The application state becomes inconsistent: orders marked as paid when payment actually failed.
// improvement: Handle all documented event types from the webhook provider. Log unhandled event types for review. Implement at least different paths for success and failure events.

app.post('/webhooks/stripe', async (req, res) => {
  const event = req.body;

  // VULNERABLE: only handles checkout.session.completed
  if (event.type === 'checkout.session.completed') {
    await handleSuccessfulPayment(event.data.object);
  }

  res.json({ received: true });
});

// VULNERABLE: webhook handler with incomplete switch
app.post('/webhooks/paypal', async (req, res) => {
  const event = req.body;

  switch (event.event_type) {
    case 'PAYMENT.CAPTURE.COMPLETED':
      await fulfillOrder(event.resource);
      break;
    // VULNERABLE: PAYMENT.CAPTURE.DENIED, PAYMENT.CAPTURE.REFUNDED, etc. not handled
  }

  res.json({ received: true });
});
