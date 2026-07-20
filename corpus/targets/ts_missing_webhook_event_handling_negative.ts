// SAFE: handle all event types
app.post('/webhooks/stripe', async (req, res) => {
  const event = req.body;

  switch (event.type) {
    case 'checkout.session.completed':
      await handleSuccessfulPayment(event.data.object);
      break;
    case 'checkout.session.expired':
      await handleExpiredSession(event.data.object);
      break;
    case 'charge.failed':
      await handleFailedCharge(event.data.object);
      break;
    case 'charge.refunded':
      await handleRefund(event.data.object);
      break;
    default:
      // SAFE: log unhandled events for review
      logger.warn({ eventType: event.type }, 'Unhandled webhook event');
  }

  res.json({ received: true });
});

app.post('/webhooks/paypal', async (req, res) => {
  const event = req.body;

  switch (event.event_type) {
    case 'PAYMENT.CAPTURE.COMPLETED':
      await fulfillOrder(event.resource);
      break;
    case 'PAYMENT.CAPTURE.DENIED':
    case 'PAYMENT.CAPTURE.REFUNDED':
    case 'PAYMENT.CAPTURE.REVERSED':
      await handleFailedPayment(event.resource);
      break;
    default:
      logger.warn({ eventType: event.event_type }, 'Unhandled PayPal event');
  }

  res.json({ received: true });
});
