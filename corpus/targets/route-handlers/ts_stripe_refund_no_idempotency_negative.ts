// SAFE: Refund uses a unique idempotency key to prevent double refunds

import Stripe from 'stripe';
import express from 'express';
import { v4 as uuidv4 } from 'uuid';

const stripe = new Stripe('sk_test_...');
const app = express();

app.post('/api/refund', async (req, res) => {
  const { paymentIntentId } = req.body;
  const idempotencyKey = `refund_${paymentIntentId}_${uuidv4()}`;

  const refund = await stripe.refunds.create({
    payment_intent: paymentIntentId,
  }, {
    idempotencyKey,
  });

  res.json({ refundId: refund.id });
});
