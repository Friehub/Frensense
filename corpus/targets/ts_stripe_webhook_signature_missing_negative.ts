// SAFE: Stripe webhook events are verified using constructEvent before processing

import express from 'express';
import Stripe from 'stripe';

const stripe = new Stripe('sk_test_...');
const app = express();
const endpointSecret = 'whsec_...';

app.post('/webhooks/stripe', express.raw({ type: 'application/json' }), async (req, res) => {
  const sig = req.headers['stripe-signature'];
  let event: Stripe.Event;

  try {
    event = stripe.webhooks.constructEvent(req.body, sig, endpointSecret);
  } catch (err) {
    return res.status(400).send(`Webhook Error: ${err.message}`);
  }

  if (event.type === 'payment_intent.succeeded') {
    await fulfillOrder(event.data.object.metadata.orderId);
  }

  res.json({ received: true });
});
