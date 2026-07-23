// SAFE: Payment method status is checked before proceeding, and setup is confirmed server-side

import Stripe from 'stripe';
import express from 'express';

const stripe = new Stripe('sk_test_...');
const app = express();

app.post('/api/charge', async (req, res) => {
  const { paymentMethodId, amount, currency } = req.body;

  const pm = await stripe.paymentMethods.retrieve(paymentMethodId);
  if (pm.type !== 'card' || pm.card?.status !== 'active') {
    return res.status(400).send('Payment method not active');
  }

  const paymentIntent = await stripe.paymentIntents.create({
    amount,
    currency,
    payment_method: paymentMethodId,
    confirm: true,
    capture_method: 'manual',
    return_url: 'https://example.com/order/confirm',
  });

  if (paymentIntent.next_action?.type === 'use_stripe_sdk') {
    return res.json({ requiresAction: true, clientSecret: paymentIntent.client_secret });
  }

  if (paymentIntent.status === 'requires_capture') {
    await stripe.paymentIntents.capture(paymentIntent.id);
    await fulfillOrder(req.body.orderId);
  }

  res.json({ status: paymentIntent.status });
});
