// [frensense]
// observation: "Payment currency is accepted from the client without validating it matches the merchant's settlement currency."
// impact: "If the client provides a currency different from the merchant account's settlement currency, Stripe auto-converts at unfavorable rates, may reject the charge, or the merchant incurs unexpected conversion fees."
// improvement: "Validate that the requested currency matches the merchant account's settlement currency, or explicitly handle multi-currency with proper conversion."

import Stripe from 'stripe';
import express from 'express';

const stripe = new Stripe('sk_test_...');
const app = express();

app.post('/api/create-payment', async (req, res) => {
  const { amount, currency, orderId } = req.body;

  const paymentIntent = await stripe.paymentIntents.create({
    amount,
    currency,
    metadata: { orderId },
  });

  res.json({ clientSecret: paymentIntent.client_secret });
});
