// [frensense]
// observation: "Payment is confirmed without first verifying the payment method was successfully attached and confirmed by the customer."
// impact: "An attacker can submit a charge using a payment method that was never authenticated (e.g., unconfirmed SetupIntent), leading to unauthorized charges or declined payments after goods are delivered."
// improvement: "Always confirm that the PaymentMethod or SetupIntent has status 'requires_capture' or 'succeeded' before fulfilling an order."

import Stripe from 'stripe';
import express from 'express';

const stripe = new Stripe('sk_test_...');
const app = express();

app.post('/api/charge', async (req, res) => {
  const { paymentMethodId, amount, currency } = req.body;

  const paymentIntent = await stripe.paymentIntents.create({
    amount,
    currency,
    payment_method: paymentMethodId,
    confirm: true,
  });

  if (paymentIntent.status === 'requires_capture' || paymentIntent.status === 'succeeded') {
    await fulfillOrder(req.body.orderId);
  }

  res.json({ status: paymentIntent.status });
});
