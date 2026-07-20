// SAFE: Uses SetupIntent flow to ensure payment method is confirmed before charging

import Stripe from 'stripe';
import express from 'express';

const stripe = new Stripe('sk_test_...');
const app = express();

app.post('/api/charge', async (req, res) => {
  const { setupIntentId, amount, currency } = req.body;

  const setupIntent = await stripe.setupIntents.retrieve(setupIntentId);
  if (setupIntent.status !== 'succeeded') {
    return res.status(400).send('Payment method not confirmed');
  }

  const paymentIntent = await stripe.paymentIntents.create({
    amount,
    currency,
    payment_method: setupIntent.payment_method as string,
    customer: setupIntent.customer as string,
    confirm: true,
  });

  if (paymentIntent.status === 'succeeded') {
    await fulfillOrder(req.body.orderId);
  }

  res.json({ status: paymentIntent.status });
});
