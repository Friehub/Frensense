// [frensense]
// observation: "Stripe webhook endpoint processes events without calling stripe.webhooks.constructEvent to verify the signature."
// impact: "An attacker who discovers the webhook URL can forge Stripe events (e.g., payment_intent.succeeded) to trigger fulfillment, refunds, or account changes without paying."
// improvement: "Always verify Stripe webhook signatures using stripe.webhooks.constructEvent with the endpoint secret."

import express from 'express';
import Stripe from 'stripe';

const stripe = new Stripe('sk_test_...');
const app = express();

app.post('/webhooks/stripe', express.raw({ type: 'application/json' }), async (req, res) => {
  const event = req.body;

  if (event.type === 'payment_intent.succeeded') {
    await fulfillOrder(event.data.object.metadata.orderId);
  }

  res.json({ received: true });
});
