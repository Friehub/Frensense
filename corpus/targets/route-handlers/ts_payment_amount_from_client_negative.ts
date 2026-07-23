// SAFE: calculate total server-side from order items
import Stripe from 'stripe';

app.post('/api/create-payment', async (req, res) => {
  const order = await db.findOrder(req.body.orderId);
  if (!order || order.userId !== req.user.id) {
    return res.status(404).json({ error: 'Order not found' });
  }

  // SAFE: total calculated from database values
  const amount = order.items.reduce((sum, item) => sum + item.price * item.quantity, 0);
  const currency = 'usd';

  const paymentIntent = await stripe.paymentIntents.create({
    amount: Math.round(amount * 100), // cents
    currency,
    metadata: { orderId: order.id },
  });

  res.json({ clientSecret: paymentIntent.client_secret });
});
