// SAFE: handle all documented event types
app.post('/webhooks/stripe', async (req, res) => {
  const event = req.body;
  switch (event.type) {
    case 'checkout.session.completed':
      await fulfillOrder(event.data.object.id);
      break;
    case 'checkout.session.expired':
      await handleExpiredSession(event.data.object);
      break;
    case 'charge.failed':
      await handleFailedCharge(event.data.object);
      break;
    case 'charge.refunded':
      await updateRefundStatus(event.data.object);
      break;
    default:
      logger.warn({ type: event.type }, 'Unhandled webhook event type');
  }
  res.json({ received: true });
});
