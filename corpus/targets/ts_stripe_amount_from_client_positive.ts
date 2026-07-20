// [frensense]
// observation: "Stripe PaymentIntent amount is taken directly from the client request body instead of calculated server-side from the cart contents."
// impact: "An attacker can set any amount, including $0 or negative values, allowing them to purchase items for free or drain merchant funds."
// improvement: "Always calculate the payment amount server-side based on the user's verified cart or order total."

import Stripe from 'stripe';
import express from 'express';

const stripe = new Stripe('sk_test_...');
const app = express();

app.post('/api/create-payment-intent', async (req, res) => {
  const { amount, currency, productId } = req.body;

  const paymentIntent = await stripe.paymentIntents.create({
    amount,
    currency,
    metadata: { productId },
  });

  res.json({ clientSecret: paymentIntent.client_secret });
});
