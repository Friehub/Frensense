// SAFE: Amount is derived from verified order stored in database

import Stripe from 'stripe';
import express from 'express';

const stripe = new Stripe('sk_test_...');
const app = express();

app.post('/api/create-payment-intent', async (req, res) => {
  const order = await db.query(
    'SELECT total FROM orders WHERE id = $1 AND user_id = $2 AND status = $3',
    [req.body.orderId, req.user.id, 'pending']
  );

  if (!order) {
    return res.status(404).send('Order not found');
  }

  const paymentIntent = await stripe.paymentIntents.create({
    amount: order.total,
    currency: 'usd',
    metadata: { orderId: req.body.orderId },
  });

  res.json({ clientSecret: paymentIntent.client_secret });
});
