// SAFE: Currency is hardcoded to match the account's settlement currency

import Stripe from 'stripe';
import express from 'express';

const stripe = new Stripe('sk_test_...');
const app = express();

const ACCOUNT_CURRENCY = 'usd';

app.post('/api/create-payment', async (req, res) => {
  const { amount, orderId } = req.body;

  const paymentIntent = await stripe.paymentIntents.create({
    amount,
    currency: ACCOUNT_CURRENCY,
    metadata: { orderId },
  });

  res.json({ clientSecret: paymentIntent.client_secret });
});
