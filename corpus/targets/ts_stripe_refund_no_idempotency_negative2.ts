// SAFE: Idempotency key is derived from the payment intent and a counter stored in the database

import Stripe from 'stripe';
import express from 'express';

const stripe = new Stripe('sk_test_...');
const app = express();

async function getNextRefundAttempt(paymentIntentId: string): Promise<number> {
  const result = await db.query(
    'UPDATE refund_counters SET count = count + 1 WHERE payment_intent_id = $1 RETURNING count',
    [paymentIntentId]
  );
  return result.count;
}

app.post('/api/refund', async (req, res) => {
  const { paymentIntentId } = req.body;
  const attempt = await getNextRefundAttempt(paymentIntentId);
  const idempotencyKey = `refund_${paymentIntentId}_${attempt}`;

  const refund = await stripe.refunds.create({
    payment_intent: paymentIntentId,
  }, {
    idempotencyKey,
  });

  res.json({ refundId: refund.id });
});
