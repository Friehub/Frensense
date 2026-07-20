// [frensense]
// observation: "Refund is processed without an idempotency key, so duplicate requests cause multiple refunds."
// impact: "If the client retries the refund request (network timeout, double-click), the same payment can be refunded multiple times, losing merchant revenue."
// improvement: "Always provide an idempotency key when creating refunds to ensure duplicate requests are safely ignored."

import Stripe from 'stripe';
import express from 'express';

const stripe = new Stripe('sk_test_...');
const app = express();

app.post('/api/refund', async (req, res) => {
  const { paymentIntentId } = req.body;

  const refund = await stripe.refunds.create({
    payment_intent: paymentIntentId,
  });

  res.json({ refundId: refund.id });
});
